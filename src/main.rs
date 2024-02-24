use axum::{
    extract::{Path, Request, State},
    http::{
        header::{self, CONTENT_TYPE},
        StatusCode,
    },
    middleware::Next,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use clap::Parser;
use dotenv::dotenv;
use files::Articles;
use std::{error::Error, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

mod files;
mod forms;
mod pages;
mod templates;

#[derive(Debug, Clone)]
pub struct AppState {
    pub debug: bool,
    pub articles: Arc<Articles>,
}

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
            let state = AppState {
                debug: true,
                articles: Arc::new(
                    Articles::from_dir("blog".into()).expect("Failed to load articles"),
                ),
            };

            let serve_router = Router::new()
                .nest_service("/", ServeDir::new("wasm").precompressed_gzip())
                .layer(axum::middleware::from_fn(no_cache_middle));

            let router = Router::new()
                .route("/", get(pages::home))
                .route("/about", get(pages::about))
                .route("/blog", get(pages::blog))
                .route("/article/:alias", get(pages::article))
                .route("/media/:alias/:file", get(serve_article_media))
                .route("/feedback", post(forms::feedback_form))
                .nest_service("/favicon.ico", ServeFile::new("favicon.ico"))
                .nest_service("/static", ServeDir::new("static"))
                .nest_service("/wasm", serve_router.into_service())
                .layer(tower_http::trace::TraceLayer::new_for_http())
                .with_state(state);

            let addr = SocketAddr::from(([127, 0, 0, 1], http_port.clone()));
            tracing::info!("Starting server on {}", addr);

            let listener = TcpListener::bind(addr).await.unwrap();
            axum::serve(listener, router).await.unwrap();
        }
    };
}

async fn no_cache_middle(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, "no-cache".parse().unwrap());
    response
}

pub enum ErrorResponse {
    FileNotFound,
    Unauthorized,
    InternalServerError(Box<dyn Error>),
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        match self {
            ErrorResponse::FileNotFound => {
                (StatusCode::NOT_FOUND, "File not found").into_response()
            }
            ErrorResponse::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
            }
            ErrorResponse::InternalServerError(err) => {
                tracing::error!("Internal server error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        }
    }
}

async fn serve_article_media(
    Path((alias, file)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Response, ErrorResponse> {
    let article = state
        .articles
        .iter()
        .find(|article| article.meta.alias == alias)
        .ok_or_else(|| ErrorResponse::FileNotFound)?;

    let file_path = article
        .files
        .get(&file)
        .ok_or_else(|| ErrorResponse::FileNotFound)?;

    let file = tokio::fs::File::open(file_path)
        .await
        .map_err(|_| ErrorResponse::FileNotFound)?;

    let mime_type = mime_guess::from_path(file_path).first_or_octet_stream();
    let stream = tokio_util::io::ReaderStream::new(file);

    let response = Response::builder()
        .header(CONTENT_TYPE, mime_type.to_string())
        .body(axum::body::Body::from_stream(stream))
        .map_err(|err| ErrorResponse::InternalServerError(err.into()))?;

    Ok(response)
}
