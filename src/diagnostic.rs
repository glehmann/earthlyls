use tower_lsp::{jsonrpc::Result, lsp_types::*};

use crate::backend::Backend;

pub mod unknown_option;

pub fn diagnostics(backend: &Backend, uri: &Url) -> Result<Vec<Diagnostic>> {
    unknown_option::unknown_option(backend, uri)
}

pub async fn publish_diagnostics(backend: &Backend, uri: &Url) -> Result<()> {
    backend.client.publish_diagnostics(uri.to_owned(), diagnostics(backend, uri)?, None).await;
    Ok(())
}
