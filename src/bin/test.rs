use earthlyls::{document::Document, queries::target_name};

fn main() -> std::io::Result<()> {
    let input = std::env::args().nth(1).unwrap();
    let content = std::fs::read_to_string(input)?;
    let doc = Document::from_str(&content);
    for node in doc.captures(target_name()) {
        println!("{}: {:?}", doc.node_content(node), node.range());
    }
    Ok(())
}
