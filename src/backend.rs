use std::path::{Path, PathBuf};
use std::time::Instant;

use clean_path::Clean;
use dashmap::DashMap;
use glob_match::glob_match;
use tower_lsp::lsp_types::request::{GotoDeclarationParams, GotoDeclarationResponse};
use tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer};
use tree_sitter::Parser;

use crate::document::Document;
use crate::error::{self, GlobResultExt, IOResultExt};
use crate::util::request_failed;

// #[derive(Debug)]
pub struct Backend {
    pub client: Client,
    pub docs: DashMap<Url, Document>,
    pub workspaces: DashMap<String, PathBuf>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_earthfile::language())
            .expect("Unable to load the earthfile language");
        Backend { client, docs: Default::default(), workspaces: Default::default() }
    }

    pub async fn load_workspaces_docs(&self) {
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

    pub fn load_workspace_docs(&self, dir: &Path) -> error::Result<()> {
        let glob_expr = dir.join("**").join("Earthfile").to_string_lossy().to_string();
        for f in glob::glob(&glob_expr).glob_ctx(&glob_expr)? {
            let path = f?;
            self.docs.insert(
                Url::from_file_path(&path)
                    .map_err(|_| error::EarthlylsError::PathToUrl { path: path.to_owned() })?,
                Document::new(&std::fs::read_to_string(&path).path_ctx(path)?),
            );
        }
        Ok(())
    }

    pub fn match_earthfile_ref(&self, origin: &Url, earthfile_ref: &str) -> Result<Vec<Url>> {
        let path = origin
            .to_file_path()
            .map_err(|_| request_failed("can't compute the earthfile path"))?;
        let path = path
            .parent()
            .ok_or_else(|| request_failed("can't compute the current Earthfile parent"))?;
        let path = path.join(earthfile_ref).join("Earthfile").clean().to_string_lossy().to_string();
        Ok(self
            .docs
            .iter()
            .map(|i| i.key().clone())
            .flat_map(|uri| {
                if glob_match(&path, &uri.to_file_path().unwrap().to_string_lossy()) {
                    Some(uri)
                } else {
                    None
                }
            })
            .collect())
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let now = Instant::now();
        // store the workspaces locations
        if let Some(workspaces) = params.workspace_folders {
            for workspace in workspaces {
                if workspace.uri.scheme() != "file" {
                    self.client
                        .log_message(MessageType::ERROR, "Unsupported workspace scheme")
                        .await;
                }
                if let Ok(path) = workspace.uri.to_file_path() {
                    self.workspaces.insert(workspace.name, path);
                } else {
                    self.client
                        .log_message(
                            MessageType::ERROR,
                            "Can't convert the workspace URI to file path",
                        )
                        .await;
                }
            }
        } else if let Some(root) = params.root_uri {
            self.workspaces.insert(
                "default".into(),
                root.to_file_path()
                    .map_err(|_| request_failed("can't compute the earthfile path"))?,
            );
        // } else if let Some(root) = params.root_path {
        //     self.workspaces.insert("default".into(), PathBuf::from_str(&root).unwrap());
        } else {
            self.client.log_message(MessageType::ERROR, "no workspace configuration").await;
        }
        self.load_workspaces_docs().await;
        self.client
            .log_message(MessageType::INFO, format!("initialize() run in {:.2?}", now.elapsed()))
            .await;
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                definition_provider: Some(OneOf::Left(true)),
                declaration_provider: Some(DeclarationCapability::Simple(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                references_provider: Some(OneOf::Left(true)),
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
                version: Some(String::from("0.2.1")),
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
        let now = Instant::now();
        self.docs.insert(
            params.text_document.uri.to_owned(),
            Document::open(&params.text_document.text),
        );
        self.client
            .log_message(MessageType::INFO, format!("did_open() run in {:.2?}", now.elapsed()))
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let now = Instant::now();
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
                self.docs.insert(uri.to_owned(), Document::open(&change.text));
            }
        }
        self.client
            .log_message(MessageType::INFO, format!("did_change() run in {:.2?}", now.elapsed()))
            .await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        if let Some(mut doc) = self.docs.get_mut(&params.text_document.uri) {
            doc.is_open = false
        };
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let now = Instant::now();
        let res = crate::commands::hover::hover(self, params);
        self.client
            .log_message(MessageType::INFO, format!("hover() run in {:.2?}", now.elapsed()))
            .await;
        res
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let now = Instant::now();
        let res = crate::commands::goto_definition::goto_definition(self, params);
        self.client
            .log_message(
                MessageType::INFO,
                format!("goto_definition() run in {:.2?}", now.elapsed()),
            )
            .await;
        res
    }

    async fn goto_declaration(
        &self,
        params: GotoDeclarationParams,
    ) -> Result<Option<GotoDeclarationResponse>> {
        let now = Instant::now();
        // declaration params and reponse are type aliases on the corresponding definition types, so we can just use
        // them as is with our goto_definition implementation
        let res = crate::commands::goto_definition::goto_definition(self, params);
        self.client
            .log_message(
                MessageType::INFO,
                format!("goto_declaration() run in {:.2?}", now.elapsed()),
            )
            .await;
        res
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let now = Instant::now();
        let res = crate::commands::references::references(self, params);
        self.client
            .log_message(MessageType::INFO, format!("references() run in {:.2?}", now.elapsed()))
            .await;
        res
    }
}
