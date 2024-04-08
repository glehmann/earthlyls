use ropey::Rope;
use tower_lsp::lsp_types::Range;
use tree_sitter::{InputEdit, Node, Point, Query, QueryCursor, Tree};

use crate::util::RopeProvider;

pub struct Document {
    pub rope: Rope,
    pub tree: Tree,
}

impl Document {
    pub fn new() -> Self {
        Document { rope: Rope::new(), tree: crate::parser::parse("", None) }
    }

    pub fn from_str(text: &str) -> Self {
        Document { rope: Rope::from_str(text), tree: crate::parser::parse(text, None) }
    }

    pub fn update(&mut self, range: Range, text: &str) {
        // char indexes to update the rope
        let start =
            self.rope.line_to_char(range.start.line as usize) + range.start.character as usize;
        let end = self.rope.line_to_char(range.end.line as usize) + range.end.character as usize;

        // byte indexes to update the tree-sitter tree
        let start_byte = self.rope.line_to_byte(range.start.line as usize)
            + self
                .rope
                .line(range.start.line as usize)
                .chars()
                .take(range.start.character as usize)
                .collect::<String>()
                .len();
        let old_end_byte = self.rope.line_to_byte(range.end.line as usize)
            + self
                .rope
                .line(range.end.line as usize)
                .chars()
                .take(range.end.character as usize)
                .collect::<String>()
                .len();
        let new_end_byte = start_byte + text.len();

        // update the rope
        // self.rope.remove(start..end);
        let end_rope = self.rope.split_off(end);
        dbg!(&end_rope);
        self.rope.split_off(start); // just discard the modified part
        let edit_rope = Rope::from_str(text);
        dbg!(&edit_rope);

        // new position to update the tree
        let end_pos_line = self.rope.len_lines() + edit_rope.len_lines() - 2;
        let end_pos_character = edit_rope.lines().last().unwrap().chars().len(); // TODO: remove that unwrap()?

        // merge the start, edited and end ropes
        self.rope.append(edit_rope);
        self.rope.append(end_rope);
        dbg!(&self.rope);

        // update the tree-sitter tree
        self.tree.edit(dbg!(&InputEdit {
            start_byte,
            old_end_byte,
            new_end_byte,
            start_position: Point::new(range.start.line as usize, range.start.character as usize),
            old_end_position: Point::new(range.end.line as usize, range.end.character as usize),
            new_end_position: Point::new(end_pos_line, end_pos_character),
        }));
        self.tree = crate::parser::parse_rope(&mut self.rope, Some(&self.tree));
        dbg!(&self.tree);
    }

    pub fn captures<'doc>(self: &'doc Self, query: &Query) -> Vec<Node<'doc>> {
        let mut query_cursor = QueryCursor::new();
        let captures =
            query_cursor.captures(&query, self.tree.root_node(), RopeProvider(self.rope.slice(..)));
        let mut res: Vec<Node<'doc>> = Vec::new();
        for (m, _) in captures {
            for c in m.captures {
                res.push(c.node);
            }
        }
        res
    }

    pub fn node_content(&self, node: Node) -> String {
        self.rope.byte_slice(node.byte_range()).to_string()
    }
}

#[cfg(test)]
mod tests {
    use tower_lsp::lsp_types::{Position, Range};

    use crate::document::Document;

    const SHORT_EARTHFILE: &str = "VERSION 0.8\n";
    const FROM_ALPINE: &str = "FROM alpine\n";
    const EARTHFILE_TREE: &str = "(source_file (version_command version: (version_major_minor)) (from_command (image_spec name: (image_name))))";

    #[test]
    fn should_create_empty() {
        let doc = Document::new();
        assert_eq!(doc.rope, "");
        assert_eq!(doc.rope.len_lines() - 1, 0);
        assert_eq!(doc.tree.root_node().to_string(), "(source_file)");
    }

    #[test]
    fn should_create_from_str() {
        let text = format!("{SHORT_EARTHFILE}{FROM_ALPINE}");
        let doc = Document::from_str(&text);
        assert_eq!(doc.rope, text[..]);
        assert_eq!(doc.rope.len_lines() - 1, 2);
        assert_eq!(doc.tree.root_node().to_string(), EARTHFILE_TREE);
    }

    #[test]
    fn should_update_at_eof() {
        let text = format!("{SHORT_EARTHFILE}{FROM_ALPINE}");
        let mut doc = Document::from_str(SHORT_EARTHFILE);
        doc.update(
            Range {
                start: Position { line: 1, character: 0 },
                end: Position { line: 1, character: 0 },
            },
            FROM_ALPINE,
        );
        assert_eq!(doc.rope, text[..]);
        assert_eq!(doc.tree.root_node().to_string(), EARTHFILE_TREE);
    }
}
