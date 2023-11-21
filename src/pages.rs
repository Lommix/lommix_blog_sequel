use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::{html, PreEscaped};
use pulldown_cmark::{Options, Parser};

use crate::AppState;
use super::layout;

pub struct PageMeta {
    pub title: String,
    pub description: String,
    pub keywords: String,
}

pub async fn home(State(state): State<Arc<AppState>>) -> Response {
    let articles = state
        .articles
        .clone()
        .map(|(path, meta)| {
            html! {
                div {
                    a href=(format!("/article/{}", meta.alias)) {
                        h1 { (meta.title) }
                    }
                    p { (meta.teaser) }
                }
            }
        })
        .collect::<Vec<_>>();

    layout::base(
        &PageMeta {
            title: "Home".into(),
            description: "Home".into(),
            keywords: "".into(),
        },
        &html!(
            @for article in articles {
                (article)
            }
        ),
    )
    .into_response()
}

pub async fn about() -> Response {
    super::layout::base(
        &PageMeta {
            title: "About".into(),
            description: "About".into(),
            keywords: "".into(),
        },
        &html!("about"),
    )
    .into_response()
}

pub async fn article(Path(alias): Path<String>, State(state): State<Arc<AppState>>) -> Response {
    let (path, meta) = match state
        .articles
        .clone()
        .filter(|(_, meta)| meta.alias == alias)
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
        div class="markdown" {
            (PreEscaped(html_output))
        }
    };

    layout::base(&meta.into(), &content).into_response()
}
