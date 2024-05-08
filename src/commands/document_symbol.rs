#![allow(deprecated)]
use std::sync::OnceLock;

use tower_lsp::{jsonrpc::Result, lsp_types::*};
use tree_sitter::{Node, Query, QueryCursor};

use crate::{
    backend::Backend,
    document::Document,
    util::{request_failed, RopeProvider, ToLSPRange},
};

pub fn document_symbol(
    backend: &Backend,
    params: DocumentSymbolParams,
) -> Result<Option<DocumentSymbolResponse>> {
    let uri = &params.text_document.uri;
    let doc = &backend.docs.get(uri).ok_or_else(|| request_failed("unknown document: {uri}"))?;

    // some query stuff
    let query = symbol_query();
    let symbol_idx = query.capture_index_for_name("symbol").unwrap();
    let mut cursor = QueryCursor::new();

    let matches = cursor.matches(query, doc.tree.root_node(), RopeProvider(doc.rope.slice(..)));
    let mut res = Vec::new();
    for m in matches {
        let Some(symbol_capture) = m.captures.iter().find(|c| c.index == symbol_idx) else {
            continue;
        };
        let name = doc.node_content(symbol_capture.node);
        res.push(SymbolInformation {
            name,
            kind: match m.pattern_index {
                0 => SymbolKind::FUNCTION,
                2 => SymbolKind::KEY,
                _ => SymbolKind::VARIABLE,
            },
            tags: None,
            deprecated: None,
            location: Location {
                uri: uri.to_owned(),
                range: symbol_capture.node.range().to_lsp_range(),
            },
            container_name: if m.pattern_index == 0 {
                None
            } else {
                container_name(doc, symbol_capture.node)
            },
        })
    }
    Ok(Some(DocumentSymbolResponse::Flat(res)))
}

fn symbol_query() -> &'static Query {
    static QUERY: OnceLock<Query> = OnceLock::new();
    QUERY.get_or_init(|| {
        Query::new(
            &crate::parser::language(),
            r"(target name: (identifier) @symbol)
              (arg_command name: (variable) @symbol)
              (env_command key: (variable) @symbol)
              (set_command name: (variable) @symbol)",
        )
        .unwrap()
    })
}

fn container_name(doc: &Document, parent: Node) -> Option<String> {
    let parent = parent.parent()?;
    if parent.grammar_name() == "target" {
        parent.child_by_field_name("name").map(|n| doc.node_content(n))
    } else {
        container_name(doc, parent)
    }
}
