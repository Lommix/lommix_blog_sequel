use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::{html, PreEscaped};
use pulldown_cmark::{Options, Parser};

use crate::AppState;

pub async fn home() -> Response {
    super::layout::base("home", &html!("test")).into_response()
}

pub async fn about() -> Response {
    super::layout::base("home", &html!("test")).into_response()
}

pub async fn article(Path(alias): Path<String>, State(state): State<Arc<AppState>>) -> Response {
    let (path, meta) = match state
        .articles
        .clone()
        .filter(|(path, meta)| meta.alias == alias)
        .nth(0)
    {
        Some(a) => a,
        None => return (StatusCode::NOT_FOUND, "Not Found").into_response(),
    };

    let markdown = tokio::fs::read_to_string(path).await.unwrap();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&markdown, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    let content = html! {
        div{
            (PreEscaped(html_output))
        }
    };

    super::layout::base("home", &content).into_response()
}
