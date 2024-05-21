use ropey::Rope;
use tree_sitter::{Language, Parser, Point, Range, Tree};

use std::sync::{Arc, Mutex, OnceLock};

fn parser() -> &'static Arc<Mutex<Parser>> {
    static HASHMAP: OnceLock<Arc<Mutex<Parser>>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_bash::language())
            .expect("Unable to load the bash language");
        Arc::new(Mutex::new(parser))
    })
}

pub fn language() -> Language {
    tree_sitter_bash::language()
}

pub fn parse(text: impl AsRef<[u8]>, old_tree: Option<&Tree>, included_ranges: &[Range]) -> Tree {
    parser()
        .lock()
        .map(|mut parser| {
            parser.set_included_ranges(included_ranges).unwrap();
            parser.parse(text, old_tree)
        })
        .unwrap()
        .unwrap()
}

pub fn parse_rope(rope: &Rope, old_tree: Option<&Tree>, included_ranges: &[Range]) -> Tree {
    parser()
        .lock()
        .map(|mut parser| {
            parser.set_included_ranges(included_ranges).unwrap();
            parser.parse_with(
                &mut |byte: usize, _pos: Point| -> &[u8] {
                    if let Some((text, byte_idx, _, _)) = rope.get_chunk_at_byte(byte) {
                        let start = byte - byte_idx;
                        &text.as_bytes()[start..]
                    } else {
                        &[]
                    }
                },
                old_tree,
            )
        })
        .unwrap()
        .unwrap()
}
