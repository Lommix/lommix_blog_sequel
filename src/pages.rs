use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::{html, PreEscaped};

use super::templates;
use crate::AppState;

pub struct PageMeta {
    pub title: String,
    pub description: String,
    pub keywords: String,
}

pub async fn home(State(state): State<Arc<AppState>>) -> Response {
    let articles = state
        .articles
        .iter()
        .map(|article| templates::article_preview(&article.meta))
        .collect::<Vec<_>>();

    templates::base(
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
    let content = super::files::read_markdown("assets/content/about.md".into())
        .await
        .unwrap();
    templates::base(
        &PageMeta {
            title: "About".into(),
            description: "About".into(),
            keywords: "".into(),
        },
        &html!((PreEscaped(&content))),
    )
    .into_response()
}

pub async fn article(Path(alias): Path<String>, State(state): State<Arc<AppState>>) -> Response {
    let mut article = match state.articles.find_by_alias(&alias) {
        Some(a) => a.clone(),
        None => return (StatusCode::NOT_FOUND, "Not Found").into_response(),
    };

    match article.compile().await {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
    }

    let content = html! {
        div class="markdown" {
            (PreEscaped(&article.compiled.unwrap()))
        }
    };

    templates::base(&article.meta.clone().into(), &content).into_response()
}
