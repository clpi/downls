use tower_lsp::lsp_types::{CompletionItem, CompletionParams, CompletionResponse};

use crate::session::DownSession;

#[tracing::instrument(level = "debug", skip(s), err)]
pub async fn completion(
    s: &DownSession,
    p: CompletionParams,
) -> anyhow::Result<Option<CompletionResponse>> {
    Ok(None)
}

#[tracing::instrument(level = "debug", skip(session), err)]
pub async fn completion_resolve(
    session: &DownSession,
    completion_item: CompletionItem,
) -> anyhow::Result<CompletionItem> {
    Ok(completion_item)
}
