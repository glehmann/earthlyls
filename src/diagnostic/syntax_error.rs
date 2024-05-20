use std::sync::OnceLock;

use tower_lsp::{jsonrpc::Result, lsp_types::*};
use tree_sitter::Query;

use crate::{document::Document, util::ToLSPRange};

pub fn syntax_error(doc: &Document) -> Result<Vec<Diagnostic>> {
    Ok(doc
        .captures(syntax_error_query())
        .iter()
        .map(|node| Diagnostic {
            range: node.range().to_lsp_range(),
            message: "syntax error".to_string(),
            severity: Some(DiagnosticSeverity::ERROR),
            ..Default::default()
        })
        .collect())
}

fn syntax_error_query() -> &'static Query {
    static QUERY: OnceLock<Query> = OnceLock::new();
    QUERY.get_or_init(|| Query::new(&crate::parser::language(), r"(ERROR) @error").unwrap())
}
