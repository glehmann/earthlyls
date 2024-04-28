use tower_lsp::{jsonrpc::Result, lsp_types::*};
use tree_sitter::Point;

use crate::{backend::Backend, descriptions::command_description, util::request_failed};

pub fn hover(backend: &Backend, params: HoverParams) -> Result<Option<Hover>> {
    let pos = &params.text_document_position_params.position;
    let uri = &params.text_document_position_params.text_document.uri;
    let tree = &backend.docs.get(uri).ok_or_else(|| request_failed("unknown document"))?.tree;
    let root_node = tree.root_node();
    let pos = Point { row: pos.line as usize, column: 1 + pos.character as usize };
    let mut cursor = root_node.walk();
    let mut description = None;
    // we only want to display the command description when hovering on a command keyword, but:
    // * the keywords are not nameds as such — they are unnamed nodes
    // * the keyword may not be the one that starts the command (hovering should work on AS in the IMPORT command)
    while cursor.goto_first_child_for_point(pos).is_some() {
        let node = cursor.node();
        // only keep unnamed nodes that looks like earthfile keywords
        if node.is_named()
            || node.is_extra()
            || !node.grammar_name().chars().all(|c| c.is_uppercase() || c == ' ')
        {
            continue;
        }
        let Some(parent) = node.parent() else {
            continue;
        };
        if let Some(d) = command_description(parent.grammar_name()) {
            description = Some(d);
        } else {
            // we may be hovering a command keyword but for a tree branch that has errors, so we search the description
            // based on the keyword and not the command node. We won’t be able to find the description when hovering
            // the AS of IMPORT in that case though
            let name = node.grammar_name().to_lowercase().replace(' ', "_") + "_command";
            if let Some(d) = command_description(&name) {
                description = Some(d);
            }
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
