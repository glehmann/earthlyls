use std::path::{Path, PathBuf};
use std::str::FromStr;

use clean_path::Clean;
use dashmap::DashMap;
use earthlyls::queries::target_name;
use earthlyls::util::{request_failed, ToLSPRange};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tree_sitter::{Node, Parser, Point};

use earthlyls::descriptions::command_description;
use earthlyls::document::Document;
use earthlyls::error::{self, GlobResultExt, IOResultExt};

// #[derive(Debug)]
struct Backend {
    client: Client,
    docs: DashMap<Url, Document>,
    workspaces: DashMap<String, PathBuf>,
}

impl Backend {
    fn new(client: Client) -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_earthfile::language())
            .expect("Unable to load the earthfile language");
        Backend { client, docs: Default::default(), workspaces: Default::default() }
    }

    async fn load_workspaces_docs(&self) {
        for item in self.workspaces.iter() {
            let dir = item.value();
            let name = item.key();
            if let Err(e) = self.load_workspace_docs(dir) {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!("Can't load {name} workspace documents: {}", e),
                    )
                    .await;
            }
        }
        dbg!(self.docs.len());
    }

    fn load_workspace_docs(&self, dir: &Path) -> error::Result<()> {
        let glob_expr = &format!("{}/**/Earthfile", dir.to_string_lossy());
        for f in glob::glob(glob_expr).glob_ctx(glob_expr)? {
            let path = f?;
            self.docs.insert(
                Url::from_file_path(&path)
                    .map_err(|_| error::EarthlylsError::PathToUrl { path: path.to_owned() })?,
                Document::from_str(&std::fs::read_to_string(&path).path_ctx(path)?),
            );
        }
        Ok(())
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        // store the workspaces locations
        if let Some(workspaces) = params.workspace_folders {
            for workspace in workspaces {
                if workspace.uri.scheme() == "file" {
                    self.workspaces
                        .insert(workspace.name, PathBuf::from_str(workspace.uri.path()).unwrap());
                } else {
                    self.client
                        .log_message(MessageType::ERROR, "Unsupported workspace scheme")
                        .await;
                }
            }
        } else if let Some(root) = params.root_uri {
            self.workspaces.insert("default".into(), PathBuf::from_str(root.path()).unwrap());
        // } else if let Some(root) = params.root_path {
        //     self.workspaces.insert("default".into(), PathBuf::from_str(&root).unwrap());
        } else {
            self.client.log_message(MessageType::ERROR, "no workspace configuration").await;
        }
        self.load_workspaces_docs().await;
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
            let mut updated = false;
            if let Some(range) = change.range {
                if let Some(mut doc) = self.docs.get_mut(&uri) {
                    doc.update(range, &change.text);
                    updated = true;
                }
            }
            if !updated {
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
        let tree = &self.docs.get(&uri).ok_or_else(|| request_failed("unknown document"))?.tree;
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
        let doc = &self.docs.get(&uri).ok_or_else(|| request_failed("unknown document"))?;
        let root_node = doc.tree.root_node();
        let pos = Point { row: pos.line as usize, column: 1 + pos.character as usize };
        let mut cursor = root_node.walk();
        let mut target_node: Option<Node> = None;
        while let Some(_) = cursor.goto_first_child_for_point(pos) {
            if ["target_ref", "function_ref"].contains(&cursor.node().grammar_name()) {
                target_node = Some(cursor.node());
                break;
            }
        }
        let Some(target_node) = target_node else {
            return Ok(None);
        };
        let Some(name_node) = target_node.child_by_field_name("name") else {
            return Ok(None);
        };
        let target_uri =
            if let Some(earthfile_ref_node) = target_node.child_by_field_name("earthfile") {
                let earthfile = doc.node_content(earthfile_ref_node);
                let path = PathBuf::from_str(uri.path())
                    .map_err(|_| request_failed("can't compute the earthfile path"))?;
                let path = path
                    .parent()
                    .ok_or_else(|| request_failed("can’t compute the current Earthfile parent"))?;
                let path = path.join(earthfile).join("Earthfile").clean();
                Url::from_file_path(path)
                    .map_err(|_| request_failed("can't convert the earthfile path to an url"))?
            } else {
                uri.to_owned()
            };
        let name = doc.node_content(name_node);
        let target_doc =
            &self.docs.get(&target_uri).ok_or_else(|| request_failed("unknown document"))?;
        for node in target_doc.captures(target_name()) {
            if target_doc.node_content(node) == name {
                return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                    uri: target_uri.to_owned(),
                    range: node.range().to_lsp_range(),
                })));
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
