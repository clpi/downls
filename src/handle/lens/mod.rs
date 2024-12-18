use tower_lsp::lsp_types::{CodeLens, CodeLensParams};

use crate::session::DownSession;

#[tracing::instrument(level = "debug", skip(s), err)]
pub async fn code_lens(s: &mut DownSession, p: CodeLensParams) -> super::Result<Vec<CodeLens>> {
    let mut lens = vec![];
    Ok(lens)
}

#[tracing::instrument(level = "debug", skip(session), err)]
pub async fn code_lens_resolve(
    session: &mut DownSession,
    lens: CodeLens,
) -> super::Result<CodeLens> {
    Ok(lens)
}
