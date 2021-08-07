use axum::{prelude::*, AddExtensionLayer};
use dotenv::dotenv;
use std::{error::Error, net::SocketAddr};

mod database;
mod env;
mod handlers;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "qarax=debug,tower_http=debug")
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let db_url = &dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set!");
    database::run_migrations(db_url).await?;
    let pool = database::connect(db_url).await?;
    let environment = env::Environment::new(pool).await?;

    let app = route("/", get(|| async { "hello" }))
        .nest("/hosts", handlers::hosts())
        .nest("/storage", handlers::storage())
        .nest("/drives", handlers::drives())
        .nest("/kernels", handlers::kernels())
        .nest("/vms", handlers::vms())
        .layer(AddExtensionLayer::new(environment))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
