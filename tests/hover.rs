mod common;

use core::panic;

use tower_lsp::lsp_types::*;

use crate::common::*;

#[tokio::test]
async fn should_hover_with_keyword() {
    let mut ctx = TestContext::new("simple");
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

#[tokio::test]
async fn should_hover_with_non_command_keyword() {
    let mut ctx = TestContext::new("simple");
    ctx.initialize().await;
    let res = ctx
        .request::<request::HoverRequest>(HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                position: Position { line: 7, character: 24 },
                text_document: TextDocumentIdentifier { uri: ctx.doc_uri("Earthfile") },
            },
            work_done_progress_params: WorkDoneProgressParams { work_done_token: None },
        })
        .await
        .unwrap();
    let HoverContents::Markup(markup) = res.contents else { panic!("not a markup content!") };
    assert!(markup.value.contains("SAVE ARTIFACT"));
    // panic!("Don’t panic!");
}

#[tokio::test]
async fn should_not_hover_outside_keyword() {
    let mut ctx = TestContext::new("simple");
    ctx.initialize().await;
    let res = ctx
        .request::<request::HoverRequest>(HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                position: Position { line: 0, character: 7 },
                text_document: TextDocumentIdentifier { uri: ctx.doc_uri("Earthfile") },
            },
            work_done_progress_params: WorkDoneProgressParams { work_done_token: None },
        })
        .await;
    assert!(res.is_none());
}

#[tokio::test]
async fn should_not_hover_on_eol() {
    let mut ctx = TestContext::new("simple");
    ctx.initialize().await;
    let res = ctx
        .request::<request::HoverRequest>(HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                position: Position { line: 0, character: 29 },
                text_document: TextDocumentIdentifier { uri: ctx.doc_uri("Earthfile") },
            },
            work_done_progress_params: WorkDoneProgressParams { work_done_token: None },
        })
        .await;
    assert!(res.is_none());
}
