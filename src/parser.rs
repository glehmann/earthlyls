use ropey::Rope;
use tree_sitter::{Parser, Point, Tree};

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

pub fn parse(text: impl AsRef<[u8]>, old_tree: Option<&Tree>) -> Tree {
    parser().lock().unwrap().parse(text, old_tree).unwrap()
}

pub fn parse_rope(rope: &Rope, old_tree: Option<&Tree>) -> Tree {
    parser()
        .lock()
        .unwrap()
        .parse_with(
            &mut |_byte: usize, pos: Point| -> String {
                // TODO: it should be possible to avoid memory allocation here by using as_str() on the current
                // chunk
                if let Some(line) = rope.get_line(pos.row as usize) {
                    line.slice(pos.column..).to_string()
                } else {
                    String::new()
                }
            },
            old_tree,
        )
        .unwrap()
}
