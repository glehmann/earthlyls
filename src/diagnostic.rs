use tower_lsp::{jsonrpc::Result, lsp_types::*};

use crate::backend::Backend;

pub mod syntax_error;
pub mod unknown_option;

pub fn diagnostics(backend: &Backend, uri: &Url) -> Result<Vec<Diagnostic>> {
    let mut ds = Vec::new();
    ds.append(&mut unknown_option::unknown_option(backend, uri)?);
    ds.append(&mut syntax_error::syntax_error(backend, uri)?);
    Ok(ds)
}

pub async fn publish_diagnostics(backend: &Backend, uri: &Url) -> Result<()> {
    backend.client.publish_diagnostics(uri.to_owned(), diagnostics(backend, uri)?, None).await;
    Ok(())
}
