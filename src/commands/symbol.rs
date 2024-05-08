use tower_lsp::{jsonrpc::Result, lsp_types::*};

use crate::backend::Backend;

use super::document_symbol::symbols;

pub fn symbol(
    backend: &Backend,
    params: WorkspaceSymbolParams,
) -> Result<Option<Vec<SymbolInformation>>> {
    Ok(Some(
        backend
            .docs
            .iter()
            .flat_map(|item| symbols(item.key(), item.value()))
            .filter(|si| si.name.contains(&params.query))
            .collect(),
    ))
}
