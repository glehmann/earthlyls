use ropey::Rope;
use tree_sitter::{Language, Parser, Point, Tree};

use std::sync::{Arc, Mutex, OnceLock};

fn parser() -> &'static Arc<Mutex<Parser>> {
    static HASHMAP: OnceLock<Arc<Mutex<Parser>>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_earthfile::language())
            .expect("Unable to load the earthfile language");
        Arc::new(Mutex::new(parser))
    })
}

pub fn language() -> Language {
    tree_sitter_earthfile::language()
}

pub fn parse(text: impl AsRef<[u8]>, old_tree: Option<&Tree>) -> Tree {
    parser().lock().unwrap().parse(text, old_tree).unwrap()
}

pub fn parse_rope(rope: &Rope, old_tree: Option<&Tree>) -> Tree {
    parser()
        .lock()
        .unwrap()
        .parse_with(
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
        .unwrap()
}
