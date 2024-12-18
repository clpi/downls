use tower_lsp::lsp_types::{
    DocumentFormattingParams, DocumentOnTypeFormattingParams, DocumentRangeFormattingParams,
    TextEdit,
};

use crate::session::DownSession;

#[tracing::instrument(level = "debug", skip(session), err)]
pub fn format(
    session: &DownSession,
    params: DocumentFormattingParams,
) -> anyhow::Result<Option<Vec<TextEdit>>> {
    let uri = params.text_document.uri.clone();
    Ok(None)
}
#[tracing::instrument(level = "debug", skip(session), err)]
pub fn format_range(
    session: &DownSession,
    params: DocumentRangeFormattingParams,
) -> anyhow::Result<Option<Vec<TextEdit>>> {
    let uri = params.text_document.uri.clone();
    Ok(None)
}

#[tracing::instrument(level = "debug", skip(session), err)]
pub fn format_on_type(
    session: &DownSession,
    params: DocumentOnTypeFormattingParams,
) -> anyhow::Result<Option<Vec<TextEdit>>> {
    let uri = params.text_document_position.text_document.uri;
    Ok(None)
}
