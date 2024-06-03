use std::sync::OnceLock;

use tower_lsp::{jsonrpc::Result, lsp_types::*};
use tree_sitter::Query;

use crate::document::Document;

pub fn missing_version(doc: &Document) -> Result<Vec<Diagnostic>> {
    let captures = doc.captures(version_query());
    // make sure to find a VERSION command at the root of the file
    let mut ok = false;
    for node in captures {
        if let Some(parent) = node.parent() {
            if parent.grammar_name() == "source_file" {
                ok = true;
            }
        }
    }
    if ok {
        Ok(Vec::new())
    } else {
        Ok(vec![Diagnostic {
            range: Range::default(),
            message: "no version specified".to_string(),
            severity: Some(DiagnosticSeverity::ERROR),
            ..Default::default()
        }])
    }
}

fn version_query() -> &'static Query {
    static QUERY: OnceLock<Query> = OnceLock::new();
    QUERY.get_or_init(|| {
        Query::new(&crate::parser::language(), r"(version_command) @version_command").unwrap()
    })
}
