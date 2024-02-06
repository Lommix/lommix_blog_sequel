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
    pub image: Option<String>,
}

pub async fn home(State(state): State<AppState>) -> Response {
    let articles = state
        .articles
        .iter()
        .map(|article| templates::article_preview(&article.meta))
        .take(3)
        .collect::<Vec<_>>();

    templates::base(
        &PageMeta {
            title: "Lommix's Blog".into(),
            description: "Gamedev, web wizardry & educational content".into(),
            keywords: "Gamedev, Webdev, Rust, Go, Neovim".into(),
            image: Some("static/images/new_banner.svg".into()),
        },
        &html!(

            div class="markdown" {
                h1 { "Welcome to my blog!" }
                p {"I am working on a game called Panzatier. It's a Top-Down Roguelike with tanks. I set up a CI Pipline to automatically deploy my current development progress to this blog using Web Assembly:"}
                wasm-frame cover="wasm/panzatier/cover.png" src="wasm/panzatier/index.html" {}
            }

            div {
                p {"It is still in early development and I am working on it in my free time. I am always looking for feedback and suggestions. If you have any, leave me a message!"}
                div class="feedback"{
                    form action="/feedback" method="post" {
                        textarea rows="4" cols="50" {}
                        input type="hidden" name="csrf" value="123" {}
                        input type="submit" value="Submit" {}
                    }
                }
            }


            h2 { "Recent Articles" }

            @for article in articles {
                (article)
            }
        ),
        None,
        None,
    )
    .into_response()
}

pub async fn about() -> Response {
    let content = super::files::read_markdown("static/content/about.md".into())
        .await
        .unwrap();
    templates::base(
        &PageMeta {
            title: "About".into(),
            description: "My name is Lorenz, a web/game developer from Germany with a passion for exploring and learning new things.".into(),
            keywords: "Gamedev, Webdev, Rust, Go, Neovim".into(),
            image: None,
        },
        &html!((PreEscaped(&content))),
        None,
        None,
    )
    .into_response()
}

pub async fn blog(State(state): State<AppState>) -> Response {
    let articles = state
        .articles
        .iter()
        .map(|article| templates::article_preview(&article.meta))
        .collect::<Vec<_>>();

    templates::base(
        &PageMeta {
            title: "Blog".into(),
            description: "Explore the intersection of game development and web development on my blog, featuring in-depth discussions and tutorials on cutting-edge technologies like Rust and Go. Enhance your coding experience with tips and tricks for Neovim, Ai, Game and Webdev.".into(),
            keywords: "Gamedev, Webdev, Rust, Go, Neovim".into(),
            image: None,
        },
        &html!(
            h1 {"Follow my recent development adventures on my Blog!"}
            hr {}
            @for article in articles {
                (article)
            }
        ),
        None,
        None,
    )
    .into_response()
}

pub async fn article(Path(alias): Path<String>, State(state): State<AppState>) -> Response {
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

    templates::base(&article.meta.clone().into(), &content, None, None).into_response()
}
