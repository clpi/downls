pub mod format;
use crate::session::DownSession;
use anyhow::Result;
use tower_lsp::lsp_types::*;
use tracing::{debug, error, field, info, trace, warn};

#[tracing::instrument(
  level = "debug",
  skip_all,
  fields(
    uri = field::display(&p.text_document.uri.as_ref()),
    language = field::display(&p.text_document.language_id),
  ),
  err,
)]
pub async fn did_open(s: &DownSession, p: DidOpenTextDocumentParams) -> anyhow::Result<()> {
    Ok(())
}

#[tracing::instrument(
  level = "debug",
  skip_all,
  fields(uri = field::display(&p.text_document.uri), version = p.text_document.version),
  err,
)]
pub async fn did_change(s: &DownSession, p: DidChangeTextDocumentParams) -> anyhow::Result<()> {
    Ok(())
}

#[tracing::instrument(
  level = "debug",
  skip_all,
  fields(uri = field::display(&p.text_document.uri)),
  err,
)]
pub async fn will_save(
    s: &DownSession,
    p: WillSaveTextDocumentParams,
) -> anyhow::Result<Vec<TextEdit>> {
    Ok(vec![])
}

#[tracing::instrument(
  level = "debug",
  skip_all,
  fields(uri = field::display(&p.text_document.uri)),
  err,
)]
pub async fn did_save(s: &DownSession, p: DidSaveTextDocumentParams) -> anyhow::Result<()> {
    Ok(())
}

#[tracing::instrument(level = "debug", skip(s), err)]
pub async fn did_close(s: &DownSession, p: DidCloseTextDocumentParams) -> anyhow::Result<()> {
    Ok(())
}
