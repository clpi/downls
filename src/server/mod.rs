pub mod capabilities;
pub mod session;
pub mod config;

use crate::{
  config::{DownConfig, DownSettings},
  handle::{

  },
  session::DownSession
};
use futures::{future::ready, FutureExt};
use serde_json::json;
use tower_lsp::{
  LanguageServer,
  jsonrpc::Result as JsonRpcResult,
  lsp_types::*
};
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex,
    },
};
use tracing::{debug, error, field, info, trace, warn};

pub struct DownServer {
    pub initialized: Arc<AtomicBool>,
    pub capabilities: ServerCapabilities,
    pub session: DownSession,
    pub sessions: Arc<Mutex<Vec<DownSession>>>,
    pub config: DownConfig,
    pub settings: DownSettings,
}
impl DownServer {
  pub fn info() -> ServerInfo {
    ServerInfo {
      name: "down".into(),
      version: Some("0.1.0-alpha".into()),
    }
  }
  pub fn capabilities() -> ServerCapabilities {
    capabilities::capabilities()
  }
}

#[tower_lsp::async_trait]
impl LanguageServer for DownServer {
    #[tracing::instrument(
      level = "debug",
      skip_all,
      err,
      fields(
        root_uri = p.root_uri.as_ref().map(field::display),
        root_path = p.root_path,
        capabilities = field::debug(&p.capabilities),
        client_info = p.client_info.as_ref().map(field::debug),
        workspace_folders = p.workspace_folders.as_ref().map(field::debug),
      )
  )]
  async fn initialize(&self, p: InitializeParams) -> JsonRpcResult<InitializeResult> {
      Ok(InitializeResult {
          capabilities: Self::capabilities(),
          server_info: Some(Self::info()),
      })
  }

  #[tracing::instrument(level = "trace", skip(self))]
  async fn initialized(&self, p: InitializedParams) {
      self.initialized.store(true, Ordering::SeqCst);
  }
  // #[tracing::instrument(level = "trace", skip(self))]
  // async fn will_save(&self, p: WillSaveTextDocumentParams) -> Result<(), tower_lsp::jsonrpc::Error> {
  //   crate::handle::will_save(&self.session, p).await.ok();
  //   Ok(())
  // }
  //
  // #[tracing::instrument(level = "trace", skip(self))]
  // async fn did_change(&self, p: DidChangeTextDocumentParams) -> JsonRpcResult<()> {
  //   crate::handle::did_change(&self.session, p).await.ok();
  //   Ok(())
  // }
  //
  // #[tracing::instrument(level = "trace", skip(self))]
  // async fn did_close(&self, p: DidCloseTextDocumentParams) -> Result<(), tower_lsp::jsonrpc::Error> {
  //   crate::handle::did_close(&self.session, p).await.ok();
  //   Ok(())
  // }
  //
  // #[tracing::instrument(level = "trace", skip(self))]
  // async fn did_save(&self, p: DidSaveTextDocumentParams) -> Result<(), tower_lsp::jsonrpc::Error> {
  //   crate::handle::did_save(&self.session, p).await.ok();
  //   Ok(())
  // }
  //
  // #[tracing::instrument(level = "trace", skip(self))]
  // async fn did_open(&self, p: DidOpenTextDocumentParams) -> Result<(), tower_lsp::jsonrpc::Error> {
  //   crate::handle::did_open(&self.session, p).await.ok();
  //   Ok(())
  // }

  #[tracing::instrument(level = "trace", skip(self))]
  async fn shutdown(&self) -> JsonRpcResult<()> {
      Ok(())
  }
}

