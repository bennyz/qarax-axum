mod vmm_service;

use clap::Clap;
use std::net::SocketAddr;
use std::time::Duration;
use tonic::transport::Server;
use vmm_service::node::node_server::NodeServer;

#[derive(Clap, Debug)]
#[clap(
    name = "qarax-node",
    rename_all = "kebab-case",
    rename_all_env = "screaming-snake"
)]
pub struct Args {
    #[clap(short, long, default_value = "50051", env)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "qarax-node=debug")
    }

    tracing_subscriber::fmt::fmt().init();

    let args = Args::parse();
    tracing::info!("Starting on port {}", args.port);
    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));

    Server::builder()
        .tcp_keepalive(Some(Duration::from_secs(60)))
        .add_service(NodeServer::new(vmm_service::VmService::default()))
        .serve(addr)
        .await?;

    Ok(())
}
