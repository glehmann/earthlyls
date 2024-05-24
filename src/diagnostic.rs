use tower_lsp::{jsonrpc::Result, lsp_types::*};

use crate::{backend::Backend, document::Document};

pub mod deprecated_build_arg;
pub mod syntax_error;
pub mod unknown_option;

pub fn doc_diagnostics(doc: &Document) -> Result<Vec<Diagnostic>> {
    let mut ds = Vec::new();
    ds.append(&mut deprecated_build_arg::deprecated_build_arg(doc)?);
    ds.append(&mut unknown_option::unknown_option(doc)?);
    ds.append(&mut syntax_error::syntax_error(doc)?);
    Ok(ds)
}

pub async fn publish_diagnostics(backend: &Backend) -> Result<()> {
    for mut item in backend.docs.iter_mut() {
        let uri = item.key().to_owned();
        let ds = doc_diagnostics(item.value())?;
        if ds != item.diagnostics {
            item.diagnostics.clone_from(&ds);
            backend.client.publish_diagnostics(uri, ds, None).await;
        }
    }
    Ok(())
}
