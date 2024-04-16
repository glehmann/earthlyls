use std::{path::PathBuf, str::FromStr};

use clean_path::Clean;
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
    let doc = &backend.docs.get(&uri).ok_or_else(|| request_failed("unknown document: {uri}"))?;
    let root_node = doc.tree.root_node();
    let pos = Point { row: pos.line as usize, column: 1 + pos.character as usize };
    let mut cursor = root_node.walk();
    let mut target_node: Option<Node> = None;
    while let Some(_) = cursor.goto_first_child_for_point(pos) {
        if ["target_ref", "function_ref"].contains(&cursor.node().grammar_name()) {
            target_node = Some(cursor.node());
            break;
        }
    }
    let Some(target_node) = target_node else {
        return Ok(None);
    };
    let Some(name_node) = target_node.child_by_field_name("name") else {
        return Ok(None);
    };
    let target_uri = if let Some(earthfile_ref_node) = target_node.child_by_field_name("earthfile")
    {
        let earthfile = doc.node_content(earthfile_ref_node);
        let path = PathBuf::from_str(uri.path())
            .map_err(|_| request_failed("can't compute the earthfile path"))?;
        let path = path
            .parent()
            .ok_or_else(|| request_failed("can't compute the current Earthfile parent"))?;
        let path = path.join(earthfile).join("Earthfile").clean();
        Url::from_file_path(path)
            .map_err(|_| request_failed("can't convert the earthfile path to an url"))?
    } else {
        uri.to_owned()
    };
    let name = doc.node_content(name_node);
    let target_doc = &backend
        .docs
        .get(&target_uri)
        .ok_or_else(|| request_failed(&format!("unknown document: {target_uri}")))?;
    for node in target_doc.captures(queries::target_name()) {
        if target_doc.node_content(node) == name {
            return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                uri: target_uri.to_owned(),
                range: node.range().to_lsp_range(),
            })));
        }
    }
    Ok(None)
}
