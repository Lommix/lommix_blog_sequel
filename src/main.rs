use axum::{
    extract::Host,
    http::{StatusCode, Uri},
    response::Redirect,
    routing::get,
    BoxError, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use clap::Parser;
use dotenv::dotenv;
use files::Articles;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};

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
    Dev,
    Prod,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let https_port: u16 = std::env::var("HTTPS_PORT")
        .expect("HTTPS_PORT must be set")
        .parse()
        .expect("bad https port");
    let http_port: u16 = std::env::var("HTTP_PORT")
        .expect("HTTP_PORT must be set")
        .parse()
        .expect("bad http port");

    match Command::parse() {
        Command::Dev => {
            let addr = SocketAddr::from(([0, 0, 0, 0], http_port.clone()));
            axum::Server::bind(&addr)
                .serve(setup_router().into_make_service())
                .await
                .unwrap();
        }
        Command::Prod => {
            let cert_path = std::env::var("SSL_CERT").expect("CERT_PATH must be set");
            let key_path = std::env::var("SSL_KEY").expect("KEY_PATH must be set");

            let config =
                RustlsConfig::from_pem_file(PathBuf::from(cert_path), PathBuf::from(key_path))
                    .await
                    .expect("failed to load cert");

            let addr = SocketAddr::from(([0, 0, 0, 0], https_port.clone()));
            let app = setup_router();

            tokio::spawn(redirect_http_to_https(https_port, http_port));
            axum_server::bind_rustls(addr, config)
                .serve(app.into_make_service())
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

// --------------------------------------
// redirect any https request to https
// --------------------------------------
async fn redirect_http_to_https(https_port: u16, http_port: u16) {
    fn make_https(
        host: String,
        uri: Uri,
        https_port: u16,
        http_port: u16,
    ) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_string = https_port.to_string();
        let http_string = http_port.to_string();
        let https_host = host.replace(http_string.as_str(), https_string.as_str());
        parts.authority = Some(https_host.parse()?);
        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, https_port, http_port) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(_) => Err(StatusCode::BAD_REQUEST),
        }
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], http_port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let router = Router::new()
        .route_service("/", get(redirect))
        .route_service("/*any", get(redirect))
        .into_make_service();

    axum::Server::from_tcp(listener.into_std().unwrap())
        .unwrap()
        .serve(router)
        .await
        .unwrap();
}
