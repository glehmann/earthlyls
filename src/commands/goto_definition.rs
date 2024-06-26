use std::sync::OnceLock;

use tower_lsp::{jsonrpc::Result, lsp_types::*};
use tree_sitter::{Node, Point, Query};

use crate::{
    backend::Backend,
    util::{request_failed, ToLSPRange},
};

pub fn goto_definition(
    backend: &Backend,
    params: GotoDefinitionParams,
) -> Result<Option<GotoDefinitionResponse>> {
    let pos = &params.text_document_position_params.position;
    let uri = &params.text_document_position_params.text_document.uri;
    let doc = &backend.docs.get(uri).ok_or_else(|| request_failed("unknown document: {uri}"))?;
    let root_node = doc.tree.root_node();
    let pos = Point { row: pos.line as usize, column: 1 + pos.character as usize };
    let mut cursor = root_node.walk();
    let mut origin_node: Option<Node> = None;
    while cursor.goto_first_child_for_point(pos).is_some() {
        if ["target_ref", "function_ref"].contains(&cursor.node().grammar_name()) {
            origin_node = Some(cursor.node());
            break;
        }
    }
    let Some(origin_node) = origin_node else {
        return Ok(None);
    };
    let Some(name_node) = origin_node.child_by_field_name("name") else {
        return Ok(None);
    };
    let target_uris = if let Some(earthfile_ref_node) = origin_node.child_by_field_name("earthfile")
    {
        let earthfile_ref = doc.node_content(earthfile_ref_node);
        backend.match_earthfile_ref(uri, &earthfile_ref)?
    } else {
        vec![uri.to_owned()]
    };
    let name = doc.node_content(name_node);
    let mut res = Vec::new();
    for target_uri in target_uris {
        let target_doc = &backend
            .docs
            .get(&target_uri)
            .ok_or_else(|| request_failed(&format!("unknown document: {target_uri}")))?;
        for node in target_doc.captures(target_name()) {
            if target_doc.node_content(node) == name {
                res.push(LocationLink {
                    origin_selection_range: Some(origin_node.range().to_lsp_range()),
                    target_uri: target_uri.to_owned(),
                    target_range: node.range().to_lsp_range(), // TODO: this should probably be a different range
                    target_selection_range: node.range().to_lsp_range(),
                });
            }
        }
    }
    Ok(Some(GotoDefinitionResponse::Link(res)))
}

fn target_name() -> &'static Query {
    static QUERY: OnceLock<Query> = OnceLock::new();
    QUERY.get_or_init(|| {
        Query::new(&crate::parser::language(), r"(target name: (identifier) @target_name)").unwrap()
    })
}
