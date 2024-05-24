use std::sync::OnceLock;

use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, Range};
use tree_sitter::{InputEdit, Node, Point, Query, QueryCursor, Tree};

use crate::util::RopeProvider;

pub struct Document {
    pub rope: Rope,
    pub tree: Tree,
    pub bash_trees: Vec<Tree>,
    pub is_open: bool,
    pub diagnostics: Vec<Diagnostic>,
}

impl Default for Document {
    fn default() -> Self {
        Document {
            rope: Rope::new(),
            tree: crate::parser::parse("", None),
            bash_trees: Vec::new(),
            is_open: false,
            diagnostics: Vec::new(),
        }
    }
}

impl Document {
    pub fn new(text: &str) -> Self {
        // just start with an empty bash tree, then get the shell fragment ranges to use with the bash parser
        // and set the actual bash tree
        let mut doc = Document {
            rope: Rope::from_str(text),
            tree: crate::parser::parse(text, None),
            bash_trees: Vec::new(),
            is_open: false,
            diagnostics: Vec::new(),
        };
        let ranges: Vec<_> =
            doc.captures(shell_fragment_query()).iter().map(|node| node.range()).collect();
        doc.bash_trees =
            ranges.iter().map(|range| crate::bash_parser::parse(text, None, &[*range])).collect();
        doc
    }

    pub fn open(text: &str) -> Self {
        let mut doc = Self::new(text);
        doc.is_open = true;
        doc
    }

    pub fn full_update(&mut self, text: &str) {
        self.rope = Rope::from_str(text);
        self.tree = crate::parser::parse(text, None);
        let ranges: Vec<_> =
            self.captures(shell_fragment_query()).iter().map(|node| node.range()).collect();
        self.bash_trees =
            ranges.iter().map(|range| crate::bash_parser::parse(text, None, &[*range])).collect();
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
        self.rope.split_off(start); // just discard the modified part
        let edit_rope = Rope::from_str(text);

        // new position to update the tree
        let end_pos_line = self.rope.len_lines() + edit_rope.len_lines() - 2;
        let end_pos_character = edit_rope.lines().last().unwrap().chars().len(); // TODO: remove that unwrap()?

        // merge the start, edited and end ropes
        self.rope.append(edit_rope);
        self.rope.append(end_rope);

        // update the tree-sitter tree
        let ie = InputEdit {
            start_byte,
            old_end_byte,
            new_end_byte,
            start_position: Point::new(range.start.line as usize, range.start.character as usize),
            old_end_position: Point::new(range.end.line as usize, range.end.character as usize),
            new_end_position: Point::new(end_pos_line, end_pos_character),
        };
        self.tree.edit(&ie);
        self.tree = crate::parser::parse_rope(&self.rope, Some(&self.tree));
        let ranges: Vec<_> =
            self.captures(shell_fragment_query()).iter().map(|node| node.range()).collect();
        self.bash_trees =
            ranges.iter().map(|range| crate::bash_parser::parse(text, None, &[*range])).collect();
    }

    pub fn captures<'doc>(&'doc self, query: &Query) -> Vec<Node<'doc>> {
        let mut query_cursor = QueryCursor::new();
        let captures =
            query_cursor.captures(query, self.tree.root_node(), RopeProvider(self.rope.slice(..)));
        let mut res: Vec<Node<'doc>> = Vec::new();
        for (m, _) in captures {
            for c in m.captures {
                res.push(c.node);
            }
        }
        res
    }

    pub fn bash_captures<'doc>(&'doc self, query: &Query) -> Vec<Node<'doc>> {
        let mut query_cursor = QueryCursor::new();
        let mut res: Vec<Node<'doc>> = Vec::new();
        for tree in self.bash_trees.iter() {
            let captures =
                query_cursor.captures(query, tree.root_node(), RopeProvider(self.rope.slice(..)));
            for (m, _) in captures {
                for c in m.captures {
                    res.push(c.node);
                }
            }
        }
        res
    }

    pub fn node_content(&self, node: Node) -> String {
        self.rope.byte_slice(node.byte_range()).to_string()
    }
}

fn shell_fragment_query() -> &'static Query {
    static QUERY: OnceLock<Query> = OnceLock::new();
    QUERY.get_or_init(|| {
        Query::new(&crate::parser::language(), r"(shell_fragment) @shell_fragment").unwrap()
    })
}

#[cfg(test)]
mod tests {
    use tower_lsp::lsp_types::{Position, Range};

    use crate::document::Document;

    const SHORT_EARTHFILE: &str = "VERSION 0.8\n";
    const FROM_ALPINE: &str = "FROM alpine\n";
    const EARTHFILE_TREE: &str = "(source_file (version_command version: (version_major_minor)) base_target: (block (from_command (image_spec name: (image_name)))))";

    #[test]
    fn should_create_empty() {
        let doc = Document::default();
        assert_eq!(doc.rope, "");
        assert_eq!(doc.rope.len_lines() - 1, 0);
        assert_eq!(doc.tree.root_node().to_string(), "(source_file)");
    }

    #[test]
    fn should_create_from_str() {
        let text = format!("{SHORT_EARTHFILE}{FROM_ALPINE}");
        let doc = Document::new(&text);
        assert_eq!(doc.rope, text[..]);
        assert_eq!(doc.rope.len_lines() - 1, 2);
        assert_eq!(doc.tree.root_node().to_string(), EARTHFILE_TREE);
    }

    #[test]
    fn should_update_at_eof() {
        let text = format!("{SHORT_EARTHFILE}{FROM_ALPINE}");
        let mut doc = Document::new(SHORT_EARTHFILE);
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
