use std::sync::OnceLock;

use tower_lsp::{jsonrpc::Result, lsp_types::*};
use tree_sitter::Query;

use crate::{document::Document, util::ToLSPRange};

pub fn deprecated_build_arg(doc: &Document) -> Result<Vec<Diagnostic>> {
    Ok(doc
        .captures(deprecated_build_arg_query())
        .iter()
        .map(|node| Diagnostic {
            range: node.range().to_lsp_range(),
            message: "--build-arg is deprecated. Use --<build-arg-key>=<build-arg-value> instead."
                .to_string(),
            severity: Some(DiagnosticSeverity::WARNING),
            ..Default::default()
        })
        .collect())
}

fn deprecated_build_arg_query() -> &'static Query {
    static QUERY: OnceLock<Query> = OnceLock::new();
    QUERY.get_or_init(|| {
        Query::new(&crate::parser::language(), r"(build_arg_deprecated) @build_arg_deprecated")
            .unwrap()
    })
}
