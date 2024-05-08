mod common;

use tower_lsp::lsp_types::*;

use crate::common::*;

#[tokio::test]
async fn should_provide_document_symbols() {
    let mut ctx = TestContext::new();
    ctx.initialize().await;
    let res = ctx
        .request::<request::DocumentSymbolRequest>(DocumentSymbolParams {
            partial_result_params: PartialResultParams { partial_result_token: None },
            text_document: TextDocumentIdentifier { uri: ctx.doc_uri("Earthfile") },
            work_done_progress_params: WorkDoneProgressParams { work_done_token: None },
        })
        .await
        .unwrap();
    let DocumentSymbolResponse::Flat(symbols) = res else {
        panic!("not a flat response!");
    };
    assert_eq!(symbols.len(), 6);
    let s = &symbols[0];
    assert_eq!(s.name, "rust");
    assert_eq!(s.kind, SymbolKind::FUNCTION);
    assert_eq!(s.location.uri, ctx.doc_uri("Earthfile"));
    assert_eq!(
        s.location.range,
        Range {
            start: Position { line: 2, character: 0 },
            end: Position { line: 2, character: 4 }
        }
    );
    assert_eq!(s.container_name, None);
    let s = &symbols[3];
    assert_eq!(s.name, "foo");
    assert_eq!(s.kind, SymbolKind::VARIABLE);
    assert_eq!(s.location.uri, ctx.doc_uri("Earthfile"));
    assert_eq!(
        s.location.range,
        Range {
            start: Position { line: 11, character: 6 },
            end: Position { line: 11, character: 9 }
        }
    );
    assert_eq!(s.container_name, Some("docker".to_string()));
    let s = &symbols[4];
    assert_eq!(s.name, "HOP");
    assert_eq!(s.kind, SymbolKind::KEY);
    assert_eq!(s.location.uri, ctx.doc_uri("Earthfile"));
    assert_eq!(
        s.location.range,
        Range {
            start: Position { line: 12, character: 6 },
            end: Position { line: 12, character: 9 }
        }
    );
    assert_eq!(s.container_name, Some("docker".to_string()));
    let s = &symbols[5];
    assert_eq!(s.name, "v");
    assert_eq!(s.kind, SymbolKind::VARIABLE);
    assert_eq!(s.location.uri, ctx.doc_uri("Earthfile"));
    assert_eq!(
        s.location.range,
        Range {
            start: Position { line: 13, character: 6 },
            end: Position { line: 13, character: 7 }
        }
    );
    assert_eq!(s.container_name, Some("docker".to_string()));
    // panic!("Don’t panic!");
}
