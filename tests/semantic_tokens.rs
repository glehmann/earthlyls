mod common;

use core::panic;

use tower_lsp::lsp_types::*;

use crate::common::*;

#[tokio::test]
async fn should_provide_full_semantic_tokens() {
    let mut ctx = TestContext::new("tokens");
    ctx.initialize().await;
    let res = ctx
        .request::<request::SemanticTokensFullRequest>(SemanticTokensParams {
            text_document: TextDocumentIdentifier { uri: ctx.doc_uri("Earthfile") },
            work_done_progress_params: WorkDoneProgressParams { work_done_token: None },
            partial_result_params: PartialResultParams { partial_result_token: None },
        })
        .await
        .unwrap();
    let SemanticTokensResult::Tokens(tokens) = res else { panic!("not a token list!") };
    let ts = tokens.data;
    assert_eq!(ts.len(), 26);
    // panic!("Don’t panic!");
}

#[tokio::test]
async fn should_provide_range_semantic_tokens() {
    let mut ctx = TestContext::new("tokens");
    ctx.initialize().await;
    let res = ctx
        .request::<request::SemanticTokensRangeRequest>(SemanticTokensRangeParams {
            text_document: TextDocumentIdentifier { uri: ctx.doc_uri("Earthfile") },
            range: Range {
                start: Position { line: 0, character: 0 },
                end: Position { line: 1, character: 0 },
            },
            work_done_progress_params: WorkDoneProgressParams { work_done_token: None },
            partial_result_params: PartialResultParams { partial_result_token: None },
        })
        .await
        .unwrap();
    let SemanticTokensRangeResult::Tokens(tokens) = res else { panic!("not a token list!") };
    let ts = tokens.data;
    assert_eq!(ts.len(), 1);
    // panic!("Don’t panic!");
}
