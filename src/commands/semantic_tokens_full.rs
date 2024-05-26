use tower_lsp::{jsonrpc::Result, lsp_types::*};

use crate::backend::Backend;

use super::semantic_tokens::compute_semantic_tokens;

pub fn semantic_tokens_full(
    backend: &Backend,
    params: SemanticTokensParams,
) -> Result<Option<SemanticTokensResult>> {
    let uri = &params.text_document.uri;
    let data = compute_semantic_tokens(backend, uri, None)?;
    Ok(Some(SemanticTokensResult::Tokens(SemanticTokens { result_id: None, data })))
}
