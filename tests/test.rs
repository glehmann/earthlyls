use earthlyls::backend::Backend;
use serde_json::Value;
use tokio::io::{duplex, AsyncReadExt, AsyncWriteExt, DuplexStream};
use tower_lsp::{LspService, Server};

pub fn req(msg: &str) -> String {
    format!("Content-Length: {}\r\n\r\n{}", msg.len(), msg)
}

// parse json rpc format
pub fn parse_jsonrpc(input: &mut &str) -> Option<Value> {
    let input_str = input.trim_start().trim_start_matches("Content-Length: ");

    let index = input_str.find('\r')?;
    let length = input_str[..index].parse::<usize>().ok()?;
    let input_str = &input_str[length.to_string().len()..];

    let input_str = input_str.trim_start_matches("\r\n\r\n");

    let body = &input_str[..length];
    let value = serde_json::from_str(&body[..length]).ok()?;
    *input = &input_str[length..];
    value
}

// A function that takes a byte slice as input and parse them to Vec<serde_json::Value>
pub fn resp(input: &[u8]) -> Vec<Value> {
    let mut input_str = std::str::from_utf8(input).unwrap();

    let mut resp_list = Vec::new();

    while let Some(val) = parse_jsonrpc(&mut input_str) {
        resp_list.push(val);
    }
    resp_list
}

#[test]
fn req_resp_should_work() {
    let req1_str = "{\"jsonrpc\":\"2.0\",\"method\":\"window/logMessage\",\"params\":{\"message\":\"Running CodeAction source.fixAll\",\"type\":4}}";
    let req2_str = "{\"jsonrpc\":\"2.0\",\"result\":[{\"edit\":{},\"isPreferred\":true,\"kind\":\"source.fixAll\",\"title\":\"Source Code fix action\"}],\"id\":1}";

    let test_buf = format!("{}{}", req(req1_str), req(req2_str));

    let resp_list = resp(test_buf.as_bytes());
    assert_eq!(
        resp_list,
        vec![
            serde_json::from_str::<Value>(req1_str).unwrap(),
            serde_json::from_str::<Value>(req2_str).unwrap()
        ]
    )
}

pub fn create_lsp() -> (DuplexStream, DuplexStream) {
    let (service, socket) = LspService::build(Backend::new).finish();
    let (req_client, req_server) = duplex(1024);
    let (resp_server, resp_client) = duplex(1024);

    // start server as concurrent task
    tokio::spawn(Server::new(req_server, resp_server, socket).serve(service));

    (req_client, resp_client)
}

pub async fn initialize_lsp(
    req_client: &mut DuplexStream,
    resp_client: &mut DuplexStream,
) -> Vec<u8> {
    let initialize = r#"{
  "jsonrpc": "2.0",
  "method": "initialize",
  "params": {
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
  },
  "id": 0
}"#;
    let mut buf = vec![0; 1024];

    req_client.write_all(req(initialize).as_bytes()).await.unwrap();
    let _ = resp_client.read(&mut buf).await.unwrap();

    buf
}

#[tokio::test]
async fn test_basic() {
    let (mut req_client, mut resp_client) = create_lsp();
    let buf = initialize_lsp(&mut req_client, &mut resp_client).await;
    assert!(!dbg!(resp(&buf)).is_empty());
}
