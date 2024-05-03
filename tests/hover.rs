mod common;

use core::panic;

use tower_lsp::lsp_types::*;

use crate::common::*;

#[tokio::test]
async fn should_hover_with_keyword() {
    let mut ctx = TestContext::new();
    ctx.initialize().await;
    let res = ctx
        .request::<request::HoverRequest>(HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                position: Position { line: 0, character: 0 },
                text_document: TextDocumentIdentifier { uri: ctx.doc_uri("Earthfile") },
            },
            work_done_progress_params: WorkDoneProgressParams { work_done_token: None },
        })
        .await
        .unwrap();
    let HoverContents::Markup(markup) = res.contents else { panic!("not a markup content!") };
    assert!(markup.value.contains("VERSION"));
    // panic!("Don’t panic!");
}
