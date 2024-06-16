use rayon::prelude::*;
use tower_lsp::{jsonrpc::Result, lsp_types::*};

use crate::{backend::Backend, document::Document};

pub mod deprecated_build_arg;
pub mod missing_version;
pub mod syntax_error;
pub mod unknown_option;

pub const SOURCE: &str = "earthlyls";

pub fn doc_diagnostics(doc: &Document) -> Result<Vec<Diagnostic>> {
    let mut ds = Vec::new();
    ds.append(&mut deprecated_build_arg::deprecated_build_arg(doc)?);
    ds.append(&mut unknown_option::unknown_option(doc)?);
    ds.append(&mut syntax_error::syntax_error(doc)?);
    ds.append(&mut missing_version::missing_version(doc)?);
    Ok(ds)
}

pub async fn publish_diagnostics(backend: &Backend) -> Result<()> {
    // decouple the collection of diagnostics to publish and the actual publishing in order to not hold a reference to
    // a dashmap element during an await call — it may lead to a dead lock
    // it may be interesting to look at alternatives like scc, memo_map, c-map, async-map, …
    // see: https://github.com/xacrimon/dashmap/issues/150
    let res = backend
        .docs
        .par_iter_mut()
        .map(|mut item| {
            let uri = item.key().to_owned();
            let ds = doc_diagnostics(item.value())?;
            if ds != item.diagnostics {
                item.diagnostics.clone_from(&ds);
                Ok(Some((uri, ds)))
            } else {
                Ok(None)
            }
        })
        .collect::<Result<Vec<_>>>()?;

    for (uri, ds) in res.into_iter().flatten() {
        backend.client.publish_diagnostics(uri, ds, None).await;
    }
    Ok(())
}
