use earthlyls::descriptions::command_description;
use tree_sitter::{Parser, Point};
use tree_sitter_earthfile;

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_earthfile::language())
        .expect("Error loading Earthfile grammar");
    let source_code = "VERSION 0.8\n";
    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();
    assert_eq!(root_node.kind(), "source_file");

    let pos = Point { row: 0, column: 0 };
    let mut cursor = root_node.walk();
    while let Some(_) = cursor.goto_first_child_for_point(pos) {
        println!("{:?}", cursor.node());
    }
    let node = cursor.node();
    println!("{:?}", command_description(node.grammar_name()));
}
