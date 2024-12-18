use tower_lsp::lsp_types::{
    CodeActionParams, CodeActionResponse, ExecuteCommandOptions, ExecuteCommandParams,
};

use crate::session::DownSession;

#[tracing::instrument(level = "debug", skip(s), err)]
pub async fn code_action(
    s: &DownSession,
    p: CodeActionParams,
) -> super::Result<Option<CodeActionResponse>> {
    Ok(None)
}

#[tracing::instrument(level = "debug", skip(s), err)]
pub async fn execute_command(s: &DownSession, p: ExecuteCommandParams) -> anyhow::Result<()> {
    Ok(())
}
#[tracing::instrument(level = "debug", skip(session), err)]
pub async fn code_action_resolve(
    session: &DownSession,
    action: CodeActionResponse,
) -> super::Result<CodeActionResponse> {
    Ok(action)
}
