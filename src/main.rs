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
use files::ArticleStore;
use lettre::message::Mailbox;
use std::{error::Error, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

mod db;
mod files;
mod htmx;
mod pages;
mod templates;

#[derive(Debug, Clone)]
pub struct AppState {
    pub debug: bool,
    pub articles: Arc<ArticleStore>,
    pub db_pool: Pool<rusqlite::Connection>,
    pub mailer: Arc<MailerConfig>,
}

#[derive(Debug, Clone)]
pub struct MailerConfig {
    smtp_user: String,
    smtp_pass: String,
    smtp_host: String,
    mail_to: Mailbox,
    mail_from: Mailbox,
}

impl MailerConfig {
    fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            smtp_user: std::env::var("SMTP_USER")?,
            smtp_pass: std::env::var("SMTP_PASS")?,
            smtp_host: std::env::var("SMTP_HOST")?,
            mail_to: std::env::var("MAIL_TO")?.parse()?,
            mail_from: std::env::var("MAIL_FROM")?.parse()?,
        })
    }
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
    let mailer = Arc::new(MailerConfig::from_env().unwrap());

    let cmd = Command::parse();
    match cmd {
        Command::Serve => {
            let state = AppState {
                debug: true,
                articles: Arc::new(
                    ArticleStore::from_dir("blog".into())
                        .await
                        .expect("Failed to load articles"),
                ),
                db_pool,
                mailer,
            };

            let serve_router = Router::new()
                .nest_service("/", ServeDir::new("wasm").precompressed_gzip())
                .layer(axum::middleware::from_fn(no_cache_middle));

            let router = Router::new()
                .route("/", get(pages::home))
                .route("/*page", get(pages::home))
                .nest("/htmx", htmx::htmx_router())
                .route("/media/:alias/:file", get(serve_article_media))
                .nest_service("/favicon.ico", ServeFile::new("favicon.ico"))
                .nest_service("/static", ServeDir::new("static").precompressed_gzip())
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
