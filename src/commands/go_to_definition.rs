use std::{path::PathBuf, str::FromStr};

use clean_path::Clean;
// use glob_match::glob_match;
use glob_match::glob_match;
use tower_lsp::{jsonrpc::Result, lsp_types::*};
use tree_sitter::{Node, Point};

use crate::{
    backend::Backend,
    queries,
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
        match_earthfile_ref(backend, uri, &earthfile_ref)?
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
        for node in target_doc.captures(queries::target_name()) {
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

fn match_earthfile_ref(backend: &Backend, origin: &Url, earthfile_ref: &str) -> Result<Vec<Url>> {
    let path = PathBuf::from_str(origin.path())
        .map_err(|_| request_failed("can't compute the earthfile path"))?;
    let path = path
        .parent()
        .ok_or_else(|| request_failed("can't compute the current Earthfile parent"))?;
    let path = path.join(earthfile_ref).join("Earthfile").clean().to_string_lossy().to_string();
    Ok(backend
        .docs
        .iter()
        .map(|i| i.key().clone())
        .flat_map(|uri| if glob_match(&path, uri.path()) { Some(uri) } else { None })
        .collect())
}
