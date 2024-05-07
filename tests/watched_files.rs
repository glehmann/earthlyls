mod common;

use std::fs::OpenOptions;
use std::io::prelude::*;

use tower_lsp::lsp_types::*;

use crate::common::*;

#[tokio::test]
async fn should_react_to_changed_notification() {
    let mut ctx = TestContext::new();
    ctx.initialize().await;

    // add a new target using +rust in ./Earthfile
    let mut f = OpenOptions::new()
        .append(true)
        .open(ctx.doc_uri("Earthfile").to_file_path().unwrap())
        .unwrap();
    write!(
        f,
        "
new-target:
    FROM +rust
"
    )
    .unwrap();
    ctx.notify::<notification::DidChangeWatchedFiles>(DidChangeWatchedFilesParams {
        changes: vec![FileEvent { uri: ctx.doc_uri("Earthfile"), typ: FileChangeType::CHANGED }],
    })
    .await;

    let res = ctx
        .request::<request::References>(ReferenceParams {
            context: ReferenceContext { include_declaration: false },
            partial_result_params: PartialResultParams { partial_result_token: None },
            text_document_position: TextDocumentPositionParams {
                position: Position { line: 2, character: 0 },
                text_document: TextDocumentIdentifier { uri: ctx.doc_uri("Earthfile") },
            },
            work_done_progress_params: WorkDoneProgressParams { work_done_token: None },
        })
        .await
        .unwrap();
    assert_eq!(res.len(), 4);
    // panic!("Don’t panic!");
}
