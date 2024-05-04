mod common;

use core::panic;

use tower_lsp::lsp_types::*;

use crate::common::*;

#[tokio::test]
async fn should_goto_definition() {
    let mut ctx = TestContext::new();
    ctx.initialize().await;
    let res = ctx
        .request::<request::GotoDefinition>(GotoDefinitionParams {
            partial_result_params: PartialResultParams { partial_result_token: None },
            text_document_position_params: TextDocumentPositionParams {
                position: Position { line: 3, character: 12 },
                text_document: TextDocumentIdentifier { uri: ctx.doc_uri("bar/Earthfile") },
            },
            work_done_progress_params: WorkDoneProgressParams { work_done_token: None },
        })
        .await
        .unwrap();
    let GotoDefinitionResponse::Link(definitions) = res else { panic!("not a link variant!") };
    assert_eq!(definitions.len(), 1);
    let definition = &definitions[0];
    assert_eq!(definition.target_uri, ctx.doc_uri("Earthfile"));
    assert_eq!(definition.target_range.start.line, 2);
    assert_eq!(definition.target_range.start.character, 0);
    assert_eq!(definition.target_range.end.line, 2);
    assert_eq!(definition.target_range.end.character, 4);
    let Some(origin_range) = definition.origin_selection_range else { panic!("no origin range!") };
    assert_eq!(origin_range.start.line, 3);
    assert_eq!(origin_range.start.character, 7);
    assert_eq!(origin_range.end.line, 3);
    assert_eq!(origin_range.end.character, 15);
    // panic!("Don’t panic!");
}
#[tokio::test]
async fn should_goto_multiple_definitions() {
    let mut ctx = TestContext::new();
    ctx.initialize().await;
    let res = ctx
        .request::<request::GotoDefinition>(GotoDefinitionParams {
            partial_result_params: PartialResultParams { partial_result_token: None },
            text_document_position_params: TextDocumentPositionParams {
                position: Position { line: 10, character: 13 },
                text_document: TextDocumentIdentifier { uri: ctx.doc_uri("Earthfile") },
            },
            work_done_progress_params: WorkDoneProgressParams { work_done_token: None },
        })
        .await
        .unwrap();
    let GotoDefinitionResponse::Link(definitions) = res else { panic!("not a link variant!") };
    assert_eq!(definitions.len(), 2);
    // panic!("Don’t panic!");
}
