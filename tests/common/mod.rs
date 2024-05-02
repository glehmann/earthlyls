#![allow(deprecated)]

use std::fmt::Debug;
use std::path::Path;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tower_lsp::lsp_types::{Url, WorkspaceFolder};
use tower_lsp::{jsonrpc, lsp_types, lsp_types::request::Request, LspService, Server};

use earthlyls::backend::Backend;

pub struct AsyncIn(UnboundedReceiver<String>);
pub struct AsyncOut(UnboundedSender<String>);

fn encode_message(content_type: Option<&str>, message: &str) -> String {
    let content_type = content_type.map(|ty| format!("\r\nContent-Type: {ty}")).unwrap_or_default();

    format!("Content-Length: {}{}\r\n\r\n{}", message.len(), content_type, message)
}

impl AsyncRead for AsyncIn {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let rx = self.get_mut();
        match rx.0.poll_recv(cx) {
            Poll::Ready(Some(v)) => {
                // tracing::debug!("read value: {:?}", v);
                buf.put_slice(v.as_bytes());
                Poll::Ready(Ok(()))
            }
            _ => Poll::Pending,
        }
    }
}

impl AsyncWrite for AsyncOut {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let tx = self.get_mut();
        let value = String::from_utf8(buf.to_vec()).unwrap();
        // tracing::debug!("write value: {value:?}");
        let _ = tx.0.send(value);
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

pub struct TestContext {
    pub request_tx: UnboundedSender<String>,
    pub response_rx: UnboundedReceiver<String>,
    pub _server: tokio::task::JoinHandle<()>,
    pub request_id: i64,
}

impl TestContext {
    pub async fn new() -> anyhow::Result<Self> {
        let (request_tx, rx) = mpsc::unbounded_channel::<String>();
        let (tx, response_rx) = mpsc::unbounded_channel::<String>();

        let async_in = AsyncIn(rx);
        let async_out = AsyncOut(tx);

        let (service, socket) = LspService::build(Backend::new).finish();
        let server = tokio::spawn(Server::new(async_in, async_out, socket).serve(service));

        Ok(Self { request_tx, response_rx, _server: server, request_id: 0 })
    }

    pub async fn send(&mut self, request: &jsonrpc::Request) -> anyhow::Result<()> {
        self.request_tx.send(encode_message(None, &serde_json::to_string(request)?))?;
        Ok(())
    }

    pub async fn recv<R: std::fmt::Debug + serde::de::DeserializeOwned>(
        &mut self,
    ) -> anyhow::Result<R> {
        // TODO split response for single messages
        loop {
            let response =
                self.response_rx.recv().await.ok_or_else(|| anyhow::anyhow!("empty response"))?;
            // decode response
            let payload = response.split('\n').last().unwrap_or_default();

            // skip log messages
            if payload.contains("window/logMessage") {
                // tracing::debug!("log: {payload}");
                continue;
            }
            let response = serde_json::from_str::<jsonrpc::Response>(payload)?;
            let (_id, result) = response.into_parts();
            return Ok(serde_json::from_value(result?)?);
        }
    }

    pub async fn request<R: Request>(&mut self, params: &R::Params) -> anyhow::Result<R::Result>
    where
        R::Result: Debug,
    {
        let request = jsonrpc::Request::build(R::METHOD)
            .id(self.request_id)
            .params(serde_json::to_value(params)?)
            .finish();
        self.request_id += 1;
        self.send(&request).await?;
        self.recv().await
    }

    pub async fn initialize(
        &mut self,
        workspace: &Path,
    ) -> anyhow::Result<<lsp_types::request::Initialize as Request>::Result> {
        // a real set of initialize param from helix. We just have to change the workspace configuration
        let initialize = r#"{
        "capabilities": {
          "general": {
            "positionEncodings": [
              "utf-8",
              "utf-32",
              "utf-16"
            ]
          },
          "textDocument": {
            "codeAction": {
              "codeActionLiteralSupport": {
                "codeActionKind": {
                  "valueSet": [
                    "",
                    "quickfix",
                    "refactor",
                    "refactor.extract",
                    "refactor.inline",
                    "refactor.rewrite",
                    "source",
                    "source.organizeImports"
                  ]
                }
              },
              "dataSupport": true,
              "disabledSupport": true,
              "isPreferredSupport": true,
              "resolveSupport": {
                "properties": [
                  "edit",
                  "command"
                ]
              }
            },
            "completion": {
              "completionItem": {
                "deprecatedSupport": true,
                "insertReplaceSupport": true,
                "resolveSupport": {
                  "properties": [
                    "documentation",
                    "detail",
                    "additionalTextEdits"
                  ]
                },
                "snippetSupport": true,
                "tagSupport": {
                  "valueSet": [
                    1
                  ]
                }
              },
              "completionItemKind": {}
            },
            "hover": {
              "contentFormat": [
                "markdown"
              ]
            },
            "inlayHint": {
              "dynamicRegistration": false
            },
            "publishDiagnostics": {
              "tagSupport": {
                "valueSet": [
                  1,
                  2
                ]
              },
              "versionSupport": true
            },
            "rename": {
              "dynamicRegistration": false,
              "honorsChangeAnnotations": false,
              "prepareSupport": true
            },
            "signatureHelp": {
              "signatureInformation": {
                "activeParameterSupport": true,
                "documentationFormat": [
                  "markdown"
                ],
                "parameterInformation": {
                  "labelOffsetSupport": true
                }
              }
            }
          },
          "window": {
            "workDoneProgress": true
          },
          "workspace": {
            "applyEdit": true,
            "configuration": true,
            "didChangeConfiguration": {
              "dynamicRegistration": false
            },
            "didChangeWatchedFiles": {
              "dynamicRegistration": true,
              "relativePatternSupport": false
            },
            "executeCommand": {
              "dynamicRegistration": false
            },
            "fileOperations": {
              "didRename": true,
              "willRename": true
            },
            "inlayHint": {
              "refreshSupport": false
            },
            "symbol": {
              "dynamicRegistration": false
            },
            "workspaceEdit": {
              "documentChanges": true,
              "failureHandling": "abort",
              "normalizesLineEndings": false,
              "resourceOperations": [
                "create",
                "rename",
                "delete"
              ]
            },
            "workspaceFolders": true
          }
        },
        "clientInfo": {
          "name": "helix",
          "version": "24.3 (109f53fb)"
        },
        "processId": 28774,
        "rootPath": "/Users/glehmann/src/earthlyls",
        "rootUri": "file:///Users/glehmann/src/earthlyls",
        "workspaceFolders": [
          {
            "name": "sdk",
            "uri": "file:///Users/glehmann/src/earthlyls"
          }
        ]
      }"#;
        let mut initialize: <lsp_types::request::Initialize as Request>::Params =
            serde_json::from_str(initialize).unwrap();
        let workspace_url = Url::from_file_path(workspace).unwrap();
        initialize.root_path = Some(workspace.to_string_lossy().to_string());
        initialize.root_uri = Some(workspace_url.clone());
        initialize.workspace_folders =
            Some(vec![WorkspaceFolder { name: "tmp".to_owned(), uri: workspace_url.clone() }]);
        self.request::<lsp_types::request::Initialize>(&initialize).await
    }
}
