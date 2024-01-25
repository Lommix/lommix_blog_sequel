use axum::{routing::get, Router};
use clap::Parser;
use dotenv::dotenv;
use files::Articles;
use std::{net::SocketAddr, sync::Arc};

mod files;
mod pages;
mod templates;

#[derive(Debug)]
pub struct AppState {
    pub debug: bool,
    pub articles: Articles,
}
unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

#[derive(Parser)]
enum Command {
    Serve,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::fmt().with_target(false).init();

    let http_port: u16 = std::env::var("HTTP_PORT")
        .expect("HTTP_PORT must be set")
        .parse()
        .expect("bad http port");

    let cmd = Command::parse();
    match cmd {
        Command::Serve => {
            let addr = SocketAddr::from(([0, 0, 0, 0], http_port.clone()));
            tracing::info!("Starting server on {}", addr);
            axum::Server::bind(&addr)
                .serve(setup_router().into_make_service())
                .await
                .unwrap();
        }
    };
}

// --------------------------------------
// build app router
// --------------------------------------
fn setup_router() -> Router {
    Router::new()
        .route("/", get(pages::home))
        .route("/about", get(pages::about))
        .route("/article/:alias", get(pages::article))
        .route("/media/:alias/:file", get(files::media))
        .route("/assets/*path", get(files::static_files))
        .route("/favicon.ico", get(files::static_files))
        .with_state(Arc::new(AppState {
            debug: true,
            articles: Articles::from_dir("assets/blog".into()).expect("Failed to load articles"),
        }))
}
