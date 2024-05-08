mod common;

use tower_lsp::lsp_types::*;

use crate::common::*;

#[tokio::test]
async fn should_provide_workspace_symbols() {
    let mut ctx = TestContext::new();
    ctx.initialize().await;
    let res = ctx
        .request::<request::WorkspaceSymbolRequest>(WorkspaceSymbolParams {
            partial_result_params: PartialResultParams { partial_result_token: None },
            query: "".to_string(),
            work_done_progress_params: WorkDoneProgressParams { work_done_token: None },
        })
        .await
        .unwrap();
    let WorkspaceSymbolResponse::Flat(symbols) = res else {
        panic!("not a flat response!");
    };
    assert_eq!(symbols.len(), 8);
    // panic!("Don’t panic!");
}

#[tokio::test]
async fn should_provide_workspace_symbols_with_query() {
    let mut ctx = TestContext::new();
    ctx.initialize().await;
    let res = ctx
        .request::<request::WorkspaceSymbolRequest>(WorkspaceSymbolParams {
            partial_result_params: PartialResultParams { partial_result_token: None },
            query: "docker".to_string(),
            work_done_progress_params: WorkDoneProgressParams { work_done_token: None },
        })
        .await
        .unwrap();
    let WorkspaceSymbolResponse::Flat(symbols) = res else {
        panic!("not a flat response!");
    };
    assert_eq!(symbols.len(), 3);
    // panic!("Don’t panic!");
}
