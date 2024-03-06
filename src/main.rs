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
use deadpool::unmanaged::Pool;
use dotenv::dotenv;
use files::Articles;
use std::{error::Error, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

mod db;
mod files;
mod forms;
mod pages;
mod templates;

#[derive(Debug, Clone)]
pub struct AppState {
    pub debug: bool,
    pub articles: Arc<Articles>,
    pub db_pool: Pool<rusqlite::Connection>,
}

#[derive(Parser)]
enum Command {
    Serve,
    Stats,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::fmt().with_target(false).init();

    let http_port: u16 = std::env::var("HTTP_PORT")
        .expect("HTTP_PORT must be set")
        .parse()
        .expect("bad http port");

    let db_pool = db::open_or_create_db().await?;

    let cmd = Command::parse();
    match cmd {
        Command::Serve => {
            let state = AppState {
                debug: true,
                articles: Arc::new(
                    Articles::from_dir("blog".into()).expect("Failed to load articles"),
                ),
                db_pool,
            };

            let serve_router = Router::new()
                .nest_service("/", ServeDir::new("wasm").precompressed_gzip())
                .layer(axum::middleware::from_fn(no_cache_middle));

            let page_router = Router::new()
                .route("/", get(pages::home))
                .route("/about", get(pages::about))
                .route("/blog", get(pages::blog))
                .route("/article/:alias", get(pages::article))
                .layer(axum::middleware::from_fn_with_state(
                    state.clone(),
                    track_clicks,
                ));

            let router = Router::new()
                .nest("/", page_router)
                .route("/media/:alias/:file", get(serve_article_media))
                .route("/feedback", post(forms::feedback_form))
                .nest_service("/favicon.ico", ServeFile::new("favicon.ico"))
                .nest_service("/static", ServeDir::new("static"))
                .nest_service("/wasm", serve_router.into_service())
                .layer(tower_http::trace::TraceLayer::new_for_http())
                .with_state(state.clone());

            let addr = SocketAddr::from(([127, 0, 0, 1], http_port.clone()));
            tracing::info!("Starting server on {}", addr);

            let listener = TcpListener::bind(addr).await.unwrap();
            axum::serve(listener, router).await.unwrap();
        }
        Command::Stats => {
            println!("printing stats ...");
            db::stats(&db_pool).await?.iter().for_each(|stat| {
                println!("{}", stat);
            });
        }
    };

    Ok(())
}

async fn no_cache_middle(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, "no-cache".parse().unwrap());
    response
}

async fn track_clicks(State(state): State<AppState>, request: Request, next: Next) -> Response {
    let page_url = request.uri().to_string();

    if let Err(err) = db::inc(&state.db_pool, page_url.as_str()).await {
        tracing::info!("cannot get con from pool, skip, {}", err);
    }

    next.run(request).await
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
