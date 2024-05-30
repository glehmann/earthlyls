use tower_lsp::{jsonrpc::Result, lsp_types::*};
use tree_sitter::Point;

use crate::{backend::Backend, util::request_failed};

pub fn completion(
    backend: &Backend,
    params: CompletionParams,
) -> Result<Option<CompletionResponse>> {
    let pos = &params.text_document_position.position;
    let uri = &params.text_document_position.text_document.uri;
    let tree = &backend.docs.get(uri).ok_or_else(|| request_failed("unknown document"))?.tree;
    let root_node = tree.root_node();
    let pos = Point { row: pos.line as usize, column: 1 + pos.character as usize };
    let mut cursor = root_node.walk();
    while cursor.goto_first_child_for_point(pos).is_some() {}
    let mut node = cursor.node();
    while !node.is_named() && node.parent().is_some() {
        node = node.parent().unwrap();
    }
    if ["target", "block"].contains(&node.grammar_name()) {
        Ok(Some(CompletionResponse::Array(
            COMMAND_KEYWORDS
                .iter()
                .map(|k| CompletionItem { label: k.to_string(), ..Default::default() })
                .collect(),
        )))
    } else {
        Ok(None)
    }
}

const COMMAND_KEYWORDS: [&str; 33] = [
    "ARG",
    "BUILD",
    "CACHE",
    "CMD",
    "COPY",
    "DO",
    "ENTRYPOINT",
    "ENV",
    "EXPOSE",
    "FOR",
    "FROM",
    "FROM DOCKERFILE",
    "FUNCTION",
    "GIT CLONE",
    "HEALTHCHECK",
    "HOST",
    "IF",
    "IMPORT",
    "LABEL",
    "LET",
    "LOCALLY",
    "PROJECT",
    "RUN",
    "SAVE ARTIFACT",
    "SAVE IMAGE",
    "SET",
    "TRY",
    "USER",
    "VERSION",
    "VOLUME",
    "WAIT",
    "WITH DOCKER",
    "WORKDIR",
];
