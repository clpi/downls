use std::sync::{Arc, Mutex};

use rsp::{
    config::{DownConfig, DownSettings},
    server::DownServer,
    session::DownSession,
};
use tower_lsp::{LspService, Server};

async fn serve() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (srvc, sock) = LspService::new(|client| DownServer {
        session: DownSession {},
        config: DownConfig {},
        settings: DownSettings {
            stop_on_disconnect: false,
        },
        capabilities: DownServer::capabilities(),
        sessions: Arc::new(Mutex::new(vec![DownSession {}])),
        initialized: Default::default(),
    });
    let serve = Server::new(stdin, stdout, sock);
    serve.serve(srvc).await;
}

#[tokio::main]
async fn main() {
    println!("hi");
    serve().await;
}
