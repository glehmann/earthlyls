use std::sync::OnceLock;

use tower_lsp::{jsonrpc::Result, lsp_types::*};
use tree_sitter::Query;

use crate::{document::Document, util::ToLSPRange};

pub fn unknown_option(doc: &Document) -> Result<Vec<Diagnostic>> {
    Ok(doc
        .captures(unknown_option_query())
        .iter()
        .map(|node| Diagnostic {
            range: node.range().to_lsp_range(),
            message: "unknown option".to_string(),
            severity: Some(DiagnosticSeverity::ERROR),
            ..Default::default()
        })
        .collect())
}

fn unknown_option_query() -> &'static Query {
    static QUERY: OnceLock<Query> = OnceLock::new();
    QUERY.get_or_init(|| {
        Query::new(&crate::parser::language(), r"(unknown_option) @unknown_option").unwrap()
    })
}
