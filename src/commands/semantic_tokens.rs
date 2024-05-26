use std::{collections::HashMap, sync::OnceLock};

use maplit::hashmap;
use tower_lsp::{jsonrpc::Result, lsp_types::*};
use tree_sitter::{Point, Query, QueryCursor};

use crate::{
    backend::Backend,
    util::{request_failed, RopeProvider, ToTSRange},
};

pub const TOKEN_TYPES: [SemanticTokenType; 8] = [
    SemanticTokenType::COMMENT,
    SemanticTokenType::FUNCTION,
    SemanticTokenType::KEYWORD,
    SemanticTokenType::OPERATOR,
    SemanticTokenType::PARAMETER,
    SemanticTokenType::PROPERTY,
    SemanticTokenType::STRING,
    SemanticTokenType::VARIABLE,
];

pub const TOKEN_MODIFIERS: [SemanticTokenModifier; 0] = [];

pub fn capture_to_token_idx() -> &'static HashMap<u32, u32> {
    static QUERY: OnceLock<HashMap<u32, u32>> = OnceLock::new();
    QUERY.get_or_init(|| {
        let query = earthfile_highlight_query();
        hashmap! {
            query.capture_index_for_name("comment").unwrap() => 0,
            query.capture_index_for_name("function").unwrap() => 1,
            query.capture_index_for_name("keyword").unwrap() => 2,
            query.capture_index_for_name("keyword.conditional").unwrap() => 2,
            query.capture_index_for_name("keyword.exception").unwrap() => 2,
            query.capture_index_for_name("keyword.import").unwrap() => 2,
            query.capture_index_for_name("keyword.repeat").unwrap() => 2,
            query.capture_index_for_name("operator").unwrap() => 3,
            query.capture_index_for_name("property").unwrap() => 5,
            query.capture_index_for_name("punctuation.bracket").unwrap() => 3,
            query.capture_index_for_name("punctuation.delimiter").unwrap() =>3,
            query.capture_index_for_name("punctuation.special").unwrap() => 3,
            query.capture_index_for_name("string").unwrap() => 6,
            query.capture_index_for_name("string.escape").unwrap() => 6,
            query.capture_index_for_name("string.special").unwrap() => 6,
            query.capture_index_for_name("variable").unwrap() => 7,
            query.capture_index_for_name("variable.parameter").unwrap() => 4,
        }
    })
}

pub fn semantic_tokens(
    backend: &Backend,
    params: SemanticTokensRangeParams,
) -> Result<Option<SemanticTokensRangeResult>> {
    let uri = &params.text_document.uri;
    let data = compute_semantic_tokens(backend, uri, Some(params.range))?;
    Ok(Some(SemanticTokensRangeResult::Tokens(SemanticTokens { result_id: None, data })))
}

pub fn compute_semantic_tokens(
    backend: &Backend,
    uri: &Url,
    range: Option<Range>,
) -> Result<Vec<SemanticToken>> {
    let doc = &backend.docs.get(uri).ok_or_else(|| request_failed("unknown document: {uri}"))?;
    let query = earthfile_highlight_query();
    let mut query_cursor = QueryCursor::new();
    if let Some(range) = range {
        query_cursor.set_point_range(range.to_ts_range());
    }
    let matches =
        query_cursor.matches(query, doc.tree.root_node(), RopeProvider(doc.rope.slice(..)));
    let mut res = Vec::new();
    let mut previous_point = Point { row: 0, column: 0 };
    let t2i = capture_to_token_idx();
    for m in matches {
        for c in m.captures {
            let range = c.node.range();
            let length = if range.start_point.row != range.end_point.row {
                doc.rope.line(range.start_point.row).len_chars() - range.start_point.column
            } else {
                range.end_point.column - range.start_point.column
            } as u32;
            res.push(SemanticToken {
                delta_line: (range.start_point.row - previous_point.row) as u32,
                delta_start: if previous_point.row == range.start_point.row {
                    range.start_point.column - previous_point.column
                } else {
                    range.start_point.column
                } as u32,
                length,
                token_type: t2i[&c.index],
                token_modifiers_bitset: 0,
            });
            previous_point = range.start_point;
        }
    }
    Ok(res)
}

pub fn earthfile_highlight_query() -> &'static Query {
    static QUERY: OnceLock<Query> = OnceLock::new();
    QUERY.get_or_init(|| {
        Query::new(&crate::parser::language(), include_str!("earthfile_highlight.scm")).unwrap()
    })
}
