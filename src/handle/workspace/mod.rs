use crate::session::DownSession;
use anyhow::{Context, Result};
use std::collections::HashMap;
use tower_lsp::lsp_types::*;

#[tracing::instrument(level = "debug", skip(s), err)]
pub async fn configure(s: &mut DownSession, p: ConfigurationParams) -> anyhow::Result<()> {
    Ok(())
}

#[tracing::instrument(level = "debug", skip(s), err)]
pub async fn execute_command(s: &mut DownSession, p: ExecuteCommandParams) -> anyhow::Result<()> {
    Ok(())
}

#[tracing::instrument(level = "debug", skip(s), err)]
pub async fn rename(s: &mut DownSession, p: RenameParams) -> anyhow::Result<Option<WorkspaceEdit>> {
    let uri = p.text_document_position.text_document.uri;
    let position = p.text_document_position.position;
    let mut changes = HashMap::new();
    let edit = WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    };
    Ok(Some(edit))
}
