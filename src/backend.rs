use std::path::{Path, PathBuf};
use std::time::Instant;

use clean_path::Clean;
use dashmap::DashMap;
use glob_match::glob_match;
use path_slash::PathExt;
use tower_lsp::lsp_types::request::{GotoDeclarationParams, GotoDeclarationResponse};
use tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer};
use tree_sitter::Parser;

use crate::document::Document;
use crate::error::{self, GlobResultExt, IOResultExt};
use crate::util::request_failed;

// #[derive(Debug)]
pub struct Backend {
    pub client: Client,
    pub version: String,
    pub docs: DashMap<Url, Document>,
    pub workspaces: DashMap<String, PathBuf>,
}

impl Backend {
    pub fn new(client: Client, version: String) -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_earthfile::language())
            .expect("Unable to load the earthfile language");
        Backend { client, version, docs: Default::default(), workspaces: Default::default() }
    }

    pub async fn load_workspaces_docs(&self) {
        for item in self.workspaces.iter() {
            let dir = item.value();
            let name = item.key();
            if let Err(e) = self.load_workspace_docs(dir) {
                self.error(format!("can't load {name} workspace documents: {}", e)).await;
            }
        }
        if let Err(e) = crate::diagnostic::publish_diagnostics(self).await {
            self.error(format!("can't publish diagnostic: {}", e)).await;
        }
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
        let path = path.join(earthfile_ref).join("Earthfile").clean().to_slash_lossy().to_string();
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

    pub async fn error(&self, message: impl AsRef<str>) {
        self.client.log_message(MessageType::ERROR, message.as_ref()).await
    }

    pub async fn warn(&self, message: impl AsRef<str>) {
        self.client.log_message(MessageType::WARNING, message.as_ref()).await
    }

    pub async fn info(&self, message: impl AsRef<str>) {
        self.client.log_message(MessageType::INFO, message.as_ref()).await
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
                    self.error("unsupported workspace scheme").await;
                    continue;
                }
                if let Ok(path) = workspace.uri.to_file_path() {
                    self.workspaces.insert(workspace.name, path);
                } else {
                    self.error("can't convert the workspace URI to file path").await;
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
            self.error("no workspace configuration").await;
        }
        self.info(format!("initialize() run in {:.2?}", now.elapsed())).await;
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                definition_provider: Some(OneOf::Left(true)),
                declaration_provider: Some(DeclarationCapability::Simple(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                references_provider: Some(OneOf::Left(true)),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            legend: SemanticTokensLegend {
                                token_types: crate::commands::semantic_tokens::TOKEN_TYPES.to_vec(),
                                token_modifiers: crate::commands::semantic_tokens::TOKEN_MODIFIERS
                                    .to_vec(),
                            },
                            full: None,
                            range: Some(true),
                            ..Default::default()
                        },
                    ),
                ),
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        change: Some(TextDocumentSyncKind::INCREMENTAL),
                        // will_save: Some(true),
                        // save: Some(TextDocumentSyncSaveOptions::Supported(true)),
                        ..Default::default()
                    },
                )),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
            server_info: Some(ServerInfo {
                name: String::from("earthlyls"),
                version: Some(String::from("0.2.2")),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        let now = Instant::now();
        self.load_workspaces_docs().await;
        self.info(format!("initialized() run in {:.2?}", now.elapsed())).await;
    }

    async fn shutdown(&self) -> Result<()> {
        self.info("earthlyls is shuting down!").await;
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let now = Instant::now();
        self.docs.insert(
            params.text_document.uri.to_owned(),
            Document::open(&params.text_document.text),
        );
        if let Err(e) = crate::diagnostic::publish_diagnostics(self).await {
            self.error(format!("can't publish diagnostic: {}", e)).await;
        }
        self.info(format!("did_open() run in {:.2?}", now.elapsed())).await;
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
                    self.info(format!("updated document {}", uri)).await;
                }
            }
            if !updated {
                self.docs.insert(uri.to_owned(), Document::open(&change.text));
                self.info(format!("created document {}", uri)).await;
            }
        }
        if let Err(e) = crate::diagnostic::publish_diagnostics(self).await {
            self.error(format!("can't publish diagnostic: {}", e)).await;
        }
        self.info(format!("did_change() run in {:.2?}", now.elapsed())).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        if let Some(mut doc) = self.docs.get_mut(&params.text_document.uri) {
            doc.is_open = false
        };
    }

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        let now = Instant::now();
        for event in params.changes {
            match event.typ {
                FileChangeType::CREATED => {
                    let Ok(path) = event.uri.to_file_path() else {
                        self.error(format!("can't convert {} to file path", event.uri)).await;
                        continue;
                    };
                    let content = match std::fs::read_to_string(&path) {
                        Ok(content) => content,
                        Err(e) => {
                            self.error(format!("can't read document {}: {:?}", &event.uri, e))
                                .await;
                            continue;
                        }
                    };
                    self.docs.insert(event.uri.to_owned(), Document::new(&content));
                    self.info(format!("loaded document {}", &event.uri)).await;
                }
                FileChangeType::CHANGED => {
                    let Ok(path) = event.uri.to_file_path() else {
                        self.error(format!("can't convert {} to file path", event.uri)).await;
                        continue;
                    };
                    let content = match std::fs::read_to_string(&path) {
                        Ok(content) => content,
                        Err(e) => {
                            self.error(format!("can't read document {}: {:?}", &event.uri, e))
                                .await;
                            continue;
                        }
                    };
                    if let Some(mut doc) = self.docs.get_mut(&event.uri) {
                        doc.full_update(&content);
                    } else {
                        self.docs.insert(event.uri.to_owned(), Document::new(&content));
                    }
                    self.info(format!("(re)loaded document {}", &event.uri)).await;
                }
                FileChangeType::DELETED => {
                    self.docs.remove(&event.uri);
                    self.info(format!("removed document {}", &event.uri)).await;
                }
                _ => self.warn(format!("unsupported file change type: {:?}", event.typ)).await,
            }
        }
        if let Err(e) = crate::diagnostic::publish_diagnostics(self).await {
            self.error(format!("can't publish diagnostics: {}", e)).await;
        }
        self.info(format!("did_change_watched_files() run in {:.2?}", now.elapsed())).await;
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let now = Instant::now();
        let res = crate::commands::hover::hover(self, params);
        self.info(format!("hover() run in {:.2?}", now.elapsed())).await;
        res
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let now = Instant::now();
        let res = crate::commands::goto_definition::goto_definition(self, params);
        self.info(format!("goto_definition() run in {:.2?}", now.elapsed())).await;
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
        self.info(format!("goto_declaration() run in {:.2?}", now.elapsed())).await;
        res
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let now = Instant::now();
        let res = crate::commands::references::references(self, params);
        self.info(format!("references() run in {:.2?}", now.elapsed())).await;
        res
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let now = Instant::now();
        let res = crate::commands::document_symbol::document_symbol(self, params);
        self.info(format!("document_symbol() run in {:.2?}", now.elapsed())).await;
        res
    }

    async fn symbol(
        &self,
        params: WorkspaceSymbolParams,
    ) -> Result<Option<Vec<SymbolInformation>>> {
        let now = Instant::now();
        let res = crate::commands::symbol::symbol(self, params);
        self.info(format!("symbol() run in {:.2?}", now.elapsed())).await;
        res
    }

    async fn semantic_tokens_range(
        &self,
        params: SemanticTokensRangeParams,
    ) -> Result<Option<SemanticTokensRangeResult>> {
        let now = Instant::now();
        let res = crate::commands::semantic_tokens::semantic_tokens(self, params);
        self.info(format!("semantic_tokens() run in {:.2?}", now.elapsed())).await;
        res
    }
}
