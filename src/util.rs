use ropey::RopeSlice;
use tower_lsp::lsp_types;
use tree_sitter::{Node, TextProvider};

/// Adapter to use a rope slice in tree-sitter queries
pub struct RopeProvider<'a>(pub RopeSlice<'a>);
impl<'a> TextProvider<&'a str> for RopeProvider<'a> {
    type I = ropey::iter::Chunks<'a>;
    fn text(&mut self, node: Node) -> Self::I {
        self.0.byte_slice(node.start_byte()..node.end_byte()).chunks()
    }
}

pub trait ToLSPRange {
    fn to_lsp_range(&self) -> lsp_types::Range;
}
impl ToLSPRange for tree_sitter::Range {
    fn to_lsp_range(&self) -> lsp_types::Range {
        lsp_types::Range {
            start: lsp_types::Position {
                line: self.start_point.row as u32,
                character: self.start_point.column as u32,
            },
            end: lsp_types::Position {
                line: self.end_point.row as u32,
                character: self.end_point.column as u32,
            },
        }
    }
}
