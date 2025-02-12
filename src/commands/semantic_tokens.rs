use std::{cmp::Ordering, collections::HashMap, sync::OnceLock};

use maplit::hashmap;
use tower_lsp::{jsonrpc::Result, lsp_types::*};
use tree_sitter::{Query, QueryCursor};

use crate::{
    backend::Backend,
    util::{request_failed, RopeProvider, ToLSPRange, ToTSRange},
};

pub const TOKEN_TYPES: [SemanticTokenType; 11] = [
    SemanticTokenType::COMMENT,
    SemanticTokenType::FUNCTION,
    SemanticTokenType::KEYWORD,
    SemanticTokenType::OPERATOR,
    SemanticTokenType::PARAMETER,
    SemanticTokenType::PROPERTY,
    SemanticTokenType::STRING,
    SemanticTokenType::VARIABLE,
    SemanticTokenType::TYPE,
    SemanticTokenType::NUMBER,
    SemanticTokenType::REGEXP,
];

pub const TOKEN_MODIFIERS: [SemanticTokenModifier; 0] = [];

fn capture_to_token_idx() -> &'static HashMap<u32, u32> {
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

fn bash_capture_to_token_idx() -> &'static HashMap<u32, u32> {
    static QUERY: OnceLock<HashMap<u32, u32>> = OnceLock::new();
    QUERY.get_or_init(|| {
        let query = bash_highlight_query();
        hashmap! {
            query.capture_index_for_name("comment").unwrap() => 0,
            query.capture_index_for_name("constant").unwrap() => 7,
            query.capture_index_for_name("constant.numeric.integer").unwrap() => 7,
            query.capture_index_for_name("function").unwrap() => 1,
            query.capture_index_for_name("keyword").unwrap() => 2,
            query.capture_index_for_name("keyword.control.conditional").unwrap() => 2,
            query.capture_index_for_name("keyword.control.repeat").unwrap() => 2,
            query.capture_index_for_name("keyword.function").unwrap() => 2,
            query.capture_index_for_name("label").unwrap() => 5,
            query.capture_index_for_name("operator").unwrap() => 3,
            query.capture_index_for_name("string").unwrap() => 6,
            query.capture_index_for_name("variable.other.member").unwrap() => 7,
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
    let mut query_cursor = QueryCursor::new();
    if let Some(range) = range {
        query_cursor.set_point_range(range.to_ts_range());
    }
    let mut overlapping = Vec::new();
    // get the tokens from the earthfile tree
    let matches = query_cursor.matches(
        earthfile_highlight_query(),
        doc.tree.root_node(),
        RopeProvider(doc.rope.slice(..)),
    );
    for m in matches {
        for c in m.captures {
            overlapping.push((c.node.range().to_lsp_range(), capture_to_token_idx()[&c.index]))
        }
    }
    // get the tokens from the bash trees
    for tree in doc.bash_trees.iter().by_ref() {
        let matches = query_cursor.matches(
            bash_highlight_query(),
            tree.root_node(),
            RopeProvider(doc.rope.slice(..)),
        );
        for m in matches {
            for c in m.captures {
                overlapping
                    .push((c.node.range().to_lsp_range(), bash_capture_to_token_idx()[&c.index]))
            }
        }
    }
    // reorder the tokens based on the start position
    overlapping.sort_by(|x, y| {
        let start_res = x.0.start.cmp(&y.0.start);
        if start_res != Ordering::Equal {
            start_res
        } else {
            x.0.end.cmp(&y.0.end)
        }
    });
    // rework the tokens to avoid any overlapping range
    let mut consecutive: Vec<(Range, u32)> = Vec::new();
    for (mut r, t) in overlapping {
        // only keep the first line of a token that covers several lines
        if r.start.line != r.end.line {
            r.end.line = r.start.line;
            r.end.character = doc.rope.line(r.start.line as usize).len_chars() as u32;
        }
        // find the tokens to update, if any
        let mut to_append: Vec<(Range, u32)> = Vec::new();
        let mut to_drop = 0;
        for (pr, pt) in consecutive.iter().rev() {
            if pr.start >= r.start {
                if pr.end <= r.end {
                    //     == previous ==
                    // ====== current =======
                    // drop the previous token
                } else {
                    //     ====== previous =======
                    // ====== current =======
                    // keep the right part of the previous token
                    let mut prr = *pr;
                    prr.start = r.end;
                    to_append.push((prr, *pt));
                }
            } else if pr.end <= r.start {
                // == previous ==
                //                  == current ==
                break;
            } else if pr.end <= r.end {
                // ====== previous =======
                //      ====== current =======
                // keep the left part of the previous token
                let mut prl = *pr;
                prl.end = r.start;
                to_append.push((prl, *pt));
            } else {
                // ====== previous =======
                //     ==== current ====
                // keep the left and the right parts of the previous token
                let mut prr = *pr;
                prr.start = r.end;
                to_append.push((prr, *pt));
                let mut prl = *pr;
                prl.end = r.start;
                to_append.push((prl, *pt));
            }
            to_drop += 1;
        }
        to_append.push((r, t));
        to_append.sort_by(|x, y| {
            let start_res = x.0.start.cmp(&y.0.start);
            if start_res != Ordering::Equal {
                start_res
            } else {
                x.0.end.cmp(&y.0.end)
            }
        });
        consecutive.truncate(consecutive.len() - to_drop);
        consecutive.append(&mut to_append);
    }
    // then compute the final result with the offset positions
    let mut res = Vec::new();
    let mut previous = Position { line: 0, character: 0 };
    for (r, t) in consecutive {
        // eprintln!("{}:{}->{}:{} {t}", r.start.line, r.start.character, r.end.line, r.end.character);
        let length = r.end.character - r.start.character;
        res.push(SemanticToken {
            delta_line: r.start.line - previous.line,
            delta_start: if previous.line == r.start.line {
                r.start.character - previous.character
            } else {
                r.start.character
            },
            length,
            token_type: t,
            token_modifiers_bitset: 0,
        });
        previous = r.start;
    }
    Ok(res)
}

fn earthfile_highlight_query() -> &'static Query {
    static QUERY: OnceLock<Query> = OnceLock::new();
    QUERY.get_or_init(|| {
        Query::new(&crate::parser::language(), include_str!("earthfile_highlight.scm")).unwrap()
    })
}

fn bash_highlight_query() -> &'static Query {
    static QUERY: OnceLock<Query> = OnceLock::new();
    QUERY.get_or_init(|| {
        Query::new(&crate::bash_parser::language(), include_str!("bash_highlight.scm")).unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::{bash_highlight_query, earthfile_highlight_query};

    #[test]
    fn should_load_earthfile_highlight_query() {
        earthfile_highlight_query();
    }

    #[test]
    fn should_load_bash_highlight_query() {
        bash_highlight_query();
    }
}
