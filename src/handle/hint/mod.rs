use tower_lsp::lsp_types::{InlayHint, InlayHintParams};

use crate::session::DownSession;

#[tracing::instrument(level = "debug", skip(session), err)]
pub async fn inlay_hint(
    session: &mut DownSession,
    params: InlayHintParams,
) -> anyhow::Result<Vec<InlayHint>> {
    let hints: Vec<InlayHint> = vec![];
    Ok(hints)
}

#[tracing::instrument(level = "debug", skip(session), err)]
pub async fn inlay_hint_resolve(
    session: &mut DownSession,
    hint: InlayHint,
) -> anyhow::Result<InlayHint> {
    Ok(hint)
}
