use std::io;
use std::{path::PathBuf, result};

use thiserror::Error;

use crate::util::request_failed;

#[derive(Error, Debug)]
pub enum EarthlylsError {
    #[error("{glob}: {source}")]
    GlobPattern { source: glob::PatternError, glob: String },

    #[error(transparent)]
    Glob(#[from] glob::GlobError),

    // #[error("IO error: {0}")]
    // Io(#[from] std::io::Error),
    #[error("{path}: {source}")]
    PathIo { path: PathBuf, source: io::Error },

    #[error("Can't convert path {path} to URL")]
    PathToUrl { path: PathBuf },
}

impl From<EarthlylsError> for tower_lsp::jsonrpc::Error {
    fn from(val: EarthlylsError) -> Self {
        request_failed(&val.to_string())
    }
}

/// Alias for a `Result` with the error type `AppError`.
pub type Result<T> = result::Result<T, EarthlylsError>;

pub trait IOResultExt<T> {
    fn path_ctx<P: Into<PathBuf>>(self, path: P) -> Result<T>;
}

impl<T> IOResultExt<T> for io::Result<T> {
    fn path_ctx<P: Into<PathBuf>>(self, path: P) -> Result<T> {
        self.map_err(|source| EarthlylsError::PathIo { source, path: path.into() })
    }
}

/// Extension trait for glob Result.
pub trait GlobResultExt<T> {
    fn glob_ctx<S: Into<String>>(self, glob: S) -> Result<T>;
}

impl<T> GlobResultExt<T> for result::Result<T, glob::PatternError> {
    fn glob_ctx<S: Into<String>>(self, glob: S) -> Result<T> {
        self.map_err(|source| EarthlylsError::GlobPattern { source, glob: glob.into() })
    }
}
