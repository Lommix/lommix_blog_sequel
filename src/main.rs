use axum::{routing::get, Router};
use clap::Parser;
use files::Articles;
use std::sync::Arc;

mod common;
mod files;
mod layout;
mod pages;

#[derive(Debug)]
pub struct AppState {
    pub debug: bool,
    pub articles: Articles,
}
unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

#[derive(Parser)]
enum Command {
    Dev,
    Prod,
}

#[tokio::main]
async fn main() {
    let cmd = Command::parse();
    match cmd {
        Command::Dev => {
            let app = Router::new()
                .route("/", get(pages::home))
                .route("/about", get(pages::about))
                .route("/article/:alias", get(pages::article))
                .route("/media/:alias/:media", get(files::media))
                .route("/assets/*path", get(files::static_files))
                .route("/favicon.ico", get(files::static_files))
                .with_state(Arc::new(AppState {
                    debug: true,
                    articles: Articles::from_dir("assets/blog".into())
                        .expect("Failed to load articles"),
                }));

            axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
        Command::Prod => {}
    };
}
