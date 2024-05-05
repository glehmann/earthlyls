use clap::{CommandFactory, Parser};
use earthlyls::{backend::Backend, cli};
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    cli::Cli::parse();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| {
        Backend::new(client, cli::Cli::command().get_version().unwrap().to_string())
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
