mod common;

use tower_lsp::lsp_types::*;

use crate::common::*;

#[tokio::test]
async fn should_publish_syntax_diagnostics() {
    let mut ctx = TestContext::new("syntax");
    ctx.initialize().await;
    let dp = ctx.recv::<PublishDiagnosticsParams>().await;
    assert_eq!(dp.uri, ctx.doc_uri("Earthfile"));
    let ds = dp.diagnostics;
    assert_eq!(ds.len(), 3);

    let d = &ds[0];
    assert_eq!(d.range.start.line, 3);
    assert_eq!(d.range.start.character, 5);
    assert_eq!(d.range.end.line, 3);
    assert_eq!(d.range.end.character, 21);
    assert_eq!(d.message, "unknown option");

    let d = &ds[1];
    assert_eq!(d.range.start.line, 4);
    assert_eq!(d.range.start.character, 2);
    assert_eq!(d.range.end.line, 4);
    assert_eq!(d.range.end.character, 6);
    assert_eq!(d.message, "syntax error");

    let d = &ds[2];
    assert_eq!(d.range.start.line, 5);
    assert_eq!(d.range.start.character, 11);
    assert_eq!(d.range.end.line, 5);
    assert_eq!(d.range.end.character, 17);
    assert_eq!(d.message, "syntax error");

    // panic!("Don’t panic!");
}
