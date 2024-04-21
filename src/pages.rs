use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::{html, Markup, PreEscaped};

use super::templates;
use crate::AppState;

pub struct PageMeta {
    pub title: String,
    pub description: String,
    pub keywords: String,
    pub image: Option<String>,
}

pub async fn home(page: Option<Path<String>>, State(state): State<AppState>) -> Response {
    templates::base(
        &meta_builder(page.as_ref().map(|p| p.as_str()), &state),
        &html!(
            @match page {
                Some(page) => {
                    div
                    hx-get=(format!("/htmx/{}", *page))
                    hx-target="#main"
                    hx-trigger="load"
                    {
                        div class="loading-spinner" src="static/images/spinner.svg" {}
                    }
                },
                None => {
                    div
                    hx-get="/htmx/home"
                    hx-target="#main"
                    hx-trigger="load"
                    {
                        div class="loading-spinner" src="static/images/spinner.svg" {}
                    }
                }
            }
        ),
    )
    .into_response()
}

fn meta_builder(page: Option<&str>, state: &AppState) -> Markup {
    let meta = page
        .map(|path| {
            let split: Option<[&str; 2]> = path.split('/').collect::<Vec<&str>>().try_into().ok();
            split
        })
        .flatten()
        .map(|[_, slug]| state.articles.find_by_alias(slug))
        .flatten()
        .map(|article| article.meta.clone().into())
        .unwrap_or(PageMeta{
            title: "Lommix's Blog".into(),
            description: "Explore the intersection of game development and web development on my blog, featuring in-depth discussions and tutorials on cutting-edge technologies like Rust and Go. Enhance your coding experience with tips and tricks for Neovim, Ai, Game and Webdev.".into(),
            keywords: "Webdev, Rust, Gamedev, Blog".into(),
            image: None,
        });

    html!(
        @if let Some(image) = &meta.image {
            meta property="og:image" content=(format!("https://lommix.com/{}", image));
        }

        title {( meta.title )}

        meta name="title" content=(meta.title);
        meta name="og:title" content=(meta.title);
        meta name="description" content=(meta.description);
        meta name="og:description" content=(meta.description);
        meta name="keywords" content=(meta.keywords);
    )
}
