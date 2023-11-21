use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Router};
use clap::Parser;
use files::BlogFsIter;
use maud::*;

mod files;
mod layout;
mod pages;

#[derive(Debug)]
pub struct AppState {
    pub debug: bool,
    pub articles: BlogFsIter,
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
                .route("/assets/*path", get(files::static_files))
                .route("/favicon.ico", get(files::static_files))
                .with_state(Arc::new(AppState {
                    debug: true,
                    articles: BlogFsIter::new("assets/blog".into()).expect("dir not found"),
                }));

            axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
        Command::Prod => {
            let app = Router::new()
                .route("/", get(hello))
                .with_state(Arc::new(AppState {
                    debug: true,
                    articles: BlogFsIter::new("assets/blog".into()).expect("dir not found"),
                }));

            axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
    };
}

async fn hello() -> axum::response::Response {
    let blog = BlogFsIter::new("assets/blog".into()).expect("dir not found");
    dbg!(&blog);

    let content = html! {
        "This is the main content of the page."
    };

    layout::base("test", &content).into_response()
}
