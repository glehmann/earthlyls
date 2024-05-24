use std::sync::OnceLock;

use tower_lsp::{jsonrpc::Result, lsp_types::*};
use tree_sitter::Query;

use crate::{document::Document, util::ToLSPRange};

pub fn syntax_error(doc: &Document) -> Result<Vec<Diagnostic>> {
    Ok(drop_nested(doc.captures(syntax_error_query()).iter().map(|node| node.range()))
        .map(|range| Diagnostic {
            range: range.to_lsp_range(),
            message: "syntax error".to_string(),
            severity: Some(DiagnosticSeverity::ERROR),
            ..Default::default()
        })
        .chain(doc.bash_captures(syntax_error_query()).iter().map(|node| Diagnostic {
            range: node.range().to_lsp_range(),
            message: "syntax error".to_string(),
            severity: Some(DiagnosticSeverity::ERROR),
            ..Default::default()
        }))
        .collect())
}

fn syntax_error_query() -> &'static Query {
    static QUERY: OnceLock<Query> = OnceLock::new();
    QUERY.get_or_init(|| Query::new(&crate::parser::language(), r"(ERROR) @error").unwrap())
}

struct DropNestedIterator<I: Iterator> {
    current: Option<tree_sitter::Range>,
    iter: I,
}
impl<I: Iterator<Item = tree_sitter::Range>> Iterator for DropNestedIterator<I> {
    type Item = tree_sitter::Range;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(current) => {
                for r in self.iter.by_ref() {
                    if r.start_byte >= current.end_byte {
                        self.current = Some(r);
                        return Some(r);
                    }
                }
                None
            }
            None => {
                self.current = self.iter.next();
                self.current
            }
        }
    }
}
fn drop_nested<I: Iterator<Item = tree_sitter::Range>>(iter: I) -> DropNestedIterator<I> {
    DropNestedIterator { current: None, iter }
}

#[cfg(test)]
mod tests {
    use super::drop_nested;

    #[test]
    fn should_drop_nested() {
        let r1 = tree_sitter::Range {
            start_byte: 0,
            end_byte: 100,
            start_point: tree_sitter::Point { row: 0, column: 0 },
            end_point: tree_sitter::Point { row: 20, column: 0 },
        };
        let r2 = tree_sitter::Range {
            start_byte: 10,
            end_byte: 15,
            start_point: tree_sitter::Point { row: 1, column: 0 },
            end_point: tree_sitter::Point { row: 2, column: 0 },
        };
        let r3 = tree_sitter::Range {
            start_byte: 130,
            end_byte: 200,
            start_point: tree_sitter::Point { row: 50, column: 0 },
            end_point: tree_sitter::Point { row: 51, column: 0 },
        };
        let input = vec![r1, r2, r3];
        let res = drop_nested(input.into_iter()).collect::<Vec<_>>();
        dbg!(&res);
        assert_eq!(res, vec![r1, r3]);
    }
}
