mod common;

use tower_lsp::lsp_types::*;

use crate::common::*;

#[tokio::test]
async fn should_provide_references() {
    let mut ctx = TestContext::new();
    ctx.initialize().await;
    let mut res = ctx
        .request::<request::References>(ReferenceParams {
            context: ReferenceContext { include_declaration: true },
            partial_result_params: PartialResultParams { partial_result_token: None },
            text_document_position: TextDocumentPositionParams {
                position: Position { line: 2, character: 2 },
                text_document: TextDocumentIdentifier { uri: ctx.doc_uri("Earthfile") },
            },
            work_done_progress_params: WorkDoneProgressParams { work_done_token: None },
        })
        .await
        .unwrap();
    res.sort_by_key(|l| (l.uri.to_string(), l.range.start.line));
    assert_eq!(res.len(), 4);
    let r = &res[0];
    assert_eq!(r.uri, ctx.doc_uri("Earthfile"));
    assert_eq!(r.range.start.line, 2);
    assert_eq!(r.range.start.character, 0);
    assert_eq!(r.range.end.line, 2);
    assert_eq!(r.range.end.character, 4);
    let r = &res[1];
    assert_eq!(r.uri, ctx.doc_uri("Earthfile"));
    assert_eq!(r.range.start.line, 6);
    assert_eq!(r.range.start.character, 7);
    assert_eq!(r.range.end.line, 6);
    assert_eq!(r.range.end.character, 12);
    let r = &res[2];
    assert_eq!(r.uri, ctx.doc_uri("bar/Earthfile"));
    assert_eq!(r.range.start.line, 3);
    assert_eq!(r.range.start.character, 7);
    assert_eq!(r.range.end.line, 3);
    assert_eq!(r.range.end.character, 15);
    let r = &res[3];
    assert_eq!(r.uri, ctx.doc_uri("foo/Earthfile"));
    assert_eq!(r.range.start.line, 3);
    assert_eq!(r.range.start.character, 7);
    assert_eq!(r.range.end.line, 3);
    assert_eq!(r.range.end.character, 15);
    // panic!("Don’t panic!");
}

#[tokio::test]
async fn should_provide_references_without_declaration() {
    let mut ctx = TestContext::new();
    ctx.initialize().await;
    let mut res = ctx
        .request::<request::References>(ReferenceParams {
            context: ReferenceContext { include_declaration: false },
            partial_result_params: PartialResultParams { partial_result_token: None },
            text_document_position: TextDocumentPositionParams {
                position: Position { line: 2, character: 2 },
                text_document: TextDocumentIdentifier { uri: ctx.doc_uri("Earthfile") },
            },
            work_done_progress_params: WorkDoneProgressParams { work_done_token: None },
        })
        .await
        .unwrap();
    res.sort_by_key(|l| (l.uri.to_string(), l.range.start.line));
    assert_eq!(res.len(), 3);
    let r = &res[0];
    assert_eq!(r.uri, ctx.doc_uri("Earthfile"));
    assert_eq!(r.range.start.line, 6);
    assert_eq!(r.range.start.character, 7);
    let r = &res[1];
    assert_eq!(r.uri, ctx.doc_uri("bar/Earthfile"));
    let r = &res[2];
    assert_eq!(r.uri, ctx.doc_uri("foo/Earthfile"));
    // panic!("Don’t panic!");
}
