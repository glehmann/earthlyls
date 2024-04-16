use std::{path::PathBuf, str::FromStr, sync::OnceLock};

use clean_path::Clean;
use tower_lsp::{jsonrpc::Result, lsp_types::*};
use tree_sitter::{Point, Query, QueryCursor};

use crate::{
    backend::Backend,
    util::{request_failed, RopeProvider, ToLSPRange},
};

pub fn references(backend: &Backend, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
    let Some((target_uri, target_name)) = get_target(backend, &params)? else {
        return Ok(None);
    };

    // some query stuff
    let query = ref_query();
    let ref_idx = query.capture_index_for_name("ref").unwrap();
    let target_earthfile_idx = query.capture_index_for_name("target_earthfile").unwrap();
    let target_name_idx = query.capture_index_for_name("target_name").unwrap();
    let mut query_cursor = QueryCursor::new();

    // now search in all the known documents to find some references to that target in that earthfile
    let mut res: Vec<Location> = Vec::new();
    for item in backend.docs.iter() {
        let other_uri = item.key();
        let other_doc = item.value();
        let matches = query_cursor.matches(
            query,
            other_doc.tree.root_node(),
            RopeProvider(other_doc.rope.slice(..)),
        );
        for m in matches {
            // extract the target name from the capture
            let Some(name_capture) =
                m.captures.iter().filter(|c| c.index == target_name_idx).nth(0)
            else {
                continue;
            };
            let ref_name = other_doc.node_content(name_capture.node);

            // extract the earthfile uri
            let earthfile_capture =
                m.captures.iter().filter(|c| c.index == target_earthfile_idx).nth(0);
            let ref_uri = if let Some(earthfile_capture) = earthfile_capture {
                let earthfile = other_doc.node_content(earthfile_capture.node);
                let path = PathBuf::from_str(other_uri.path())
                    .map_err(|_| request_failed("can't compute the earthfile path"))?;
                let path = path
                    .parent()
                    .ok_or_else(|| request_failed("can't compute the current Earthfile parent"))?;
                let path = path.join(earthfile).join("Earthfile").clean();
                Url::from_file_path(path)
                    .map_err(|_| request_failed("can't convert the earthfile path to an url"))?
            } else {
                other_uri.to_owned()
            };
            if ref_uri == target_uri && ref_name == target_name {
                let range = if let Some(ref_capture) =
                    m.captures.iter().filter(|c| c.index == ref_idx).nth(0)
                {
                    ref_capture.node.range()
                } else {
                    name_capture.node.range()
                };

                res.push(Location { uri: other_uri.to_owned(), range: range.to_lsp_range() });
            }
        }
    }
    Ok(Some(res))
}

fn get_target(backend: &Backend, params: &ReferenceParams) -> Result<Option<(Url, String)>> {
    let pos = &params.text_document_position.position;
    let uri = &params.text_document_position.text_document.uri;
    // let include_declaration = &params.context.include_declaration;
    let doc = &backend.docs.get(&uri).ok_or_else(|| request_failed("unknown document: {uri}"))?;
    let pos = Point { row: pos.line as usize, column: pos.character as usize };

    // some query stuff
    let query = target_and_ref_query();
    let ref_idx = query.capture_index_for_name("ref").unwrap();
    let target_earthfile_idx = query.capture_index_for_name("target_earthfile").unwrap();
    let target_name_idx = query.capture_index_for_name("target_name").unwrap();

    // find the query match at the given position
    let mut query_cursor = QueryCursor::new();
    let mut matches =
        query_cursor.matches(query, doc.tree.root_node(), RopeProvider(doc.rope.slice(..)));
    let Some(m) = matches.find(|m| {
        let node = m
            .nodes_for_capture_index(ref_idx)
            .nth(0)
            .or_else(|| m.nodes_for_capture_index(target_name_idx).nth(0))
            .unwrap();
        node.start_position() <= pos && pos < node.end_position()
    }) else {
        return Ok(None);
    };

    // extract the target name from the capture
    let Some(name_capture) = m.captures.iter().filter(|c| c.index == target_name_idx).nth(0) else {
        return Ok(None);
    };
    let target_name = doc.node_content(name_capture.node);

    // extract the earthfile uri
    let earthfile_capture = m.captures.iter().filter(|c| c.index == target_earthfile_idx).nth(0);
    let target_uri = if let Some(earthfile_capture) = earthfile_capture {
        let earthfile = doc.node_content(earthfile_capture.node);
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
    Ok(Some((target_uri, target_name)))
}

pub fn target_and_ref_query() -> &'static Query {
    static QUERY: OnceLock<Query> = OnceLock::new();
    QUERY.get_or_init(|| {
        Query::new(
            &crate::parser::language(),
            r"(target_ref
                earthfile: (earthfile_ref)? @target_earthfile
                name: (identifier) @target_name) @ref
              (function_ref
                earthfile: (earthfile_ref)? @target_earthfile
                name: (identifier) @target_name) @ref
              (target name: (identifier) @target_name)
              ",
        )
        .unwrap()
    })
}

pub fn ref_query() -> &'static Query {
    static QUERY: OnceLock<Query> = OnceLock::new();
    QUERY.get_or_init(|| {
        Query::new(
            &crate::parser::language(),
            r"(target_ref
                earthfile: (earthfile_ref)? @target_earthfile
                name: (identifier) @target_name) @ref
              (function_ref
                earthfile: (earthfile_ref)? @target_earthfile
                name: (identifier) @target_name) @ref
              ",
        )
        .unwrap()
    })
}
