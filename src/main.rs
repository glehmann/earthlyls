use dashmap::DashMap;
use earthlyls::queries::target_name;
use earthlyls::util::ToLSPRange;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tree_sitter::{Parser, Point};

use earthlyls::descriptions::command_description;
use earthlyls::document::Document;

// #[derive(Debug)]
struct Backend {
    client: Client,
    docs: DashMap<Url, Document>,
}

impl Backend {
    fn new(client: Client) -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_earthfile::language())
            .expect("Unable to load the earthfile language");
        Backend { client, docs: Default::default() }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                definition_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        change: Some(TextDocumentSyncKind::INCREMENTAL),
                        // will_save: Some(true),
                        // save: Some(TextDocumentSyncSaveOptions::Supported(true)),
                        ..Default::default()
                    },
                )),
                ..ServerCapabilities::default()
            },
            server_info: Some(ServerInfo {
                name: String::from("earthlyls"),
                version: Some(String::from("0.1.0")),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client.log_message(MessageType::INFO, "earthlyls initialized!").await;
    }

    async fn shutdown(&self) -> Result<()> {
        self.client.log_message(MessageType::INFO, "earthlyls is shuting down!").await;
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.docs.insert(
            params.text_document.uri.to_owned(),
            Document::from_str(&params.text_document.text),
        );
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        for change in params.content_changes {
            if let Some(range) = change.range {
                self.docs.get_mut(&uri).unwrap().update(range, &change.text); // TODO: remove the unwrap()?
            } else {
                self.docs.insert(uri.to_owned(), Document::from_str(&change.text));
            }
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.docs.remove(&params.text_document.uri);
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let pos = &params.text_document_position_params.position;
        let uri = &params.text_document_position_params.text_document.uri;
        let tree = &self.docs.get(&uri).unwrap().tree; // FIXME: we should actually deal with the error
        let root_node = tree.root_node();
        let pos = Point { row: pos.line as usize, column: 1 + pos.character as usize };
        // search a description to show to the user
        let mut cursor = root_node.walk();
        let mut description = None;
        while let Some(_) = cursor.goto_first_child_for_point(pos) {
            let name = cursor.node().grammar_name();
            if let Some(d) = command_description(&name) {
                description = Some(d);
            }
        }
        if let Some(description) = description {
            let markup_content =
                MarkupContent { kind: MarkupKind::Markdown, value: description.to_owned() };
            let hover_contents = HoverContents::Markup(markup_content);
            let hover = Hover { contents: hover_contents, range: None };
            Ok(Some(hover))
        } else {
            Ok(None)
        }
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let pos = &params.text_document_position_params.position;
        let uri = &params.text_document_position_params.text_document.uri;
        let doc = &self.docs.get(&uri).unwrap(); // FIXME: we should actually deal with the error
        let root_node = doc.tree.root_node();
        let pos = Point { row: pos.line as usize, column: 1 + pos.character as usize };
        let mut cursor = root_node.walk();
        while let Some(_) = cursor.goto_first_child_for_point(pos) {
            if cursor.node().grammar_name() == "target_ref" {
                let node = cursor.node();
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = doc.rope.slice(name_node.byte_range()).to_string();
                    for node in doc.captures(target_name()) {
                        if doc.node_content(node) == name {
                            return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                                uri: uri.to_owned(),
                                range: node.range().to_lsp_range(),
                            })));
                        }
                    }
                }
            }
        }
        Ok(None)
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
