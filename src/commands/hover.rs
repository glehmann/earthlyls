use tower_lsp::{jsonrpc::Result, lsp_types::*};
use tree_sitter::Point;

use crate::{backend::Backend, descriptions::command_description, util::request_failed};

pub fn hover(backend: &Backend, params: HoverParams) -> Result<Option<Hover>> {
    let pos = &params.text_document_position_params.position;
    let uri = &params.text_document_position_params.text_document.uri;
    let tree = &backend.docs.get(uri).ok_or_else(|| request_failed("unknown document"))?.tree;
    let root_node = tree.root_node();
    let pos = Point { row: pos.line as usize, column: 1 + pos.character as usize };
    // search a description to show to the user
    let mut cursor = root_node.walk();
    let mut description = None;
    while cursor.goto_first_child_for_point(pos).is_some() {
        let node = cursor.node();
        let Some(parent) = node.parent() else {
            continue;
        };
        if node.is_named()
            || node.is_extra()
            || !node.grammar_name().chars().all(|c| c.is_uppercase() || c.is_whitespace())
        {
            continue;
        }
        if let Some(d) = command_description(parent.grammar_name()) {
            description = Some(d);
        }
    }
    if let Some(description) = description {
        let markup_content =
            MarkupContent { kind: MarkupKind::Markdown, value: description.to_owned() };
        let hover_contents = HoverContents::Markup(markup_content);
        let hover = Hover { contents: hover_contents, range: None };
        Ok(Some(hover))
    } else {
        Ok(None)
    }
}
