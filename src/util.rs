use clean_path::Clean;
use glob_match::glob_match;
use path_slash::PathBufExt;
use ropey::RopeSlice;
use tower_lsp::{
    jsonrpc::{Error, Result},
    lsp_types::{self, Url},
};
use tree_sitter::{Node, TextProvider};

/// Adapter to use a rope slice in tree-sitter queries
pub struct RopeProvider<'a>(pub RopeSlice<'a>);
impl<'a> TextProvider<&'a str> for RopeProvider<'a> {
    type I = ropey::iter::Chunks<'a>;
    fn text(&mut self, node: Node) -> Self::I {
        self.0.byte_slice(node.start_byte()..node.end_byte()).chunks()
    }
}

pub trait ToLSPRange {
    fn to_lsp_range(&self) -> lsp_types::Range;
}
impl ToLSPRange for tree_sitter::Range {
    fn to_lsp_range(&self) -> lsp_types::Range {
        lsp_types::Range {
            start: lsp_types::Position {
                line: self.start_point.row as u32,
                character: self.start_point.column as u32,
            },
            end: lsp_types::Position {
                line: self.end_point.row as u32,
                character: self.end_point.column as u32,
            },
        }
    }
}

pub trait ToTSRange {
    fn to_ts_range(&self) -> std::ops::Range<tree_sitter::Point>;
}
impl ToTSRange for lsp_types::Range {
    fn to_ts_range(&self) -> std::ops::Range<tree_sitter::Point> {
        std::ops::Range {
            start: tree_sitter::Point {
                row: self.start.line as usize,
                column: self.start.character as usize,
            },
            end: tree_sitter::Point {
                row: self.end.line as usize,
                column: self.end.character as usize,
            },
        }
    }
}

pub fn request_failed(msg: &str) -> Error {
    Error {
        code: tower_lsp::jsonrpc::ErrorCode::ServerError(-32803),
        message: std::borrow::Cow::Owned(msg.to_owned()),
        data: None,
    }
}

pub fn is_earthfile_ref_match(origin: &Url, earthfile_ref: &str, target_uri: &Url) -> Result<bool> {
    let path =
        origin.to_file_path().map_err(|_| request_failed("can't compute the earthfile path"))?;
    let path = path
        .parent()
        .ok_or_else(|| request_failed("can't compute the current Earthfile parent"))?;
    let path = path.join(earthfile_ref).join("Earthfile").clean().to_slash_lossy().to_string();
    let target_path = target_uri
        .to_file_path()
        .map_err(|_| request_failed("can't compute the earthfile path"))?
        .to_string_lossy()
        .to_string();
    Ok(glob_match(&path, &target_path))
}
