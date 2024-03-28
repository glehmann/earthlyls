use std::sync::Arc;

use dashmap::DashMap;
use earthlyls::descriptions::command_description;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tree_sitter::{Parser, Point, Tree};

// #[derive(Debug)]
struct Backend {
    client: Client,
    doc_trees: DashMap<Url, Tree>,
    parser: Arc<Mutex<Parser>>,
}

impl Backend {
    fn new(client: Client) -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_earthfile::language())
            .expect("Unable to load the earthfile language");
        let parser = Arc::new(Mutex::new(parser));
        Backend { client, doc_trees: Default::default(), parser }
    }

    async fn add_doc(&self, url: &Url, content: &str) {
        let tree = self.parser.lock().await.parse(content, None).unwrap(); // TODO: check what can make the parser completely fail, and maybe deal with the possible failure
        self.doc_trees.insert(url.to_owned(), tree);
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
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
        self.client
            .log_message(
                MessageType::INFO,
                format!("adding {:?} doc", params.text_document.uri.path()),
            )
            .await;
        self.add_doc(&params.text_document.uri, &params.text_document.text).await;
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let pos = params.text_document_position_params.position;
        let uri = &params.text_document_position_params.text_document.uri;
        let tree = self.doc_trees.get(&uri).unwrap(); // FIXME: we should actually deal with the error
        let root_node = tree.root_node();
        let pos = Point { row: pos.line as usize, column: pos.character as usize };
        let mut cursor = root_node.walk();
        while let Some(_) = cursor.goto_first_child_for_point(pos) {
            self.client.log_message(MessageType::INFO, format!("{:?}", cursor.node())).await;
        }
        let name = cursor.node().grammar_name();
        if let Some(description) = command_description(name) {
            let markup_content =
                MarkupContent { kind: MarkupKind::Markdown, value: description.to_owned() };
            let hover_contents = HoverContents::Markup(markup_content);
            let hover = Hover { contents: hover_contents, range: None };
            Ok(Some(hover))
        } else {
            Ok(None)
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
