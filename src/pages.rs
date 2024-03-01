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
            description:"Discover the creative world of Lommix, a dedicated web and game developer. Immerse yourself in his interactive games and gain valuable insights through informative articles on various aspects of development. Explore now on Lommix's Blog – your go-to hub for development knowledge and entertainment.".into(),
            keywords: "Gamedev, Webdev, Rust, Go, Neovim".into(),
            image: Some("static/images/new_banner.svg".into()),
        },
        &html!(
            h1 { "Welcome! Develop with me!" }

            hr {};

            p {"I am building a game called Panzatier.
                It's a Top-Down sci-fi Roguelike written in Rust using the Bevy Engine.
                I have set up a CI Pipline to automatically deploy my current development progress
                to this blog using Web Assembly. It is very raw and sometimes may be broken,
                depending on what I am currently working on."}

            wasm-frame cover="wasm/panzatier/cover.png" src="wasm/panzatier/index.html" {}

            div {
                p class="error" {"The new Bevy 0.13 update broke most custom shaders for webassembly. So no shadows and lights for now. Expecting a fix soon."}

                h3 {"Controls:"}

                p {"[ WASD ] : Move [ MouseWheel ]: ZOOM [ Hold Space ]: drift [ Shift ]: Dash"}

                p {"It's still at the beginning stages and I'm working on it when I have spare time. WebGL has its limits and some things like particles and compute shaders don't work in the browser. So this is a simpler version of the game. If you have any thoughts or ideas, drop me a note!"}

                p{"If you want to see more, checkout my devlogs on youtube! " a target="_blank" href="https://www.youtube.com/watch?v=bvf0Nm2idyQ" {"My latest video"}}

                form class="feedback" action="/feedback" method="post" {
                    textarea name="message" rows="4" cols="50" {}
                    input class="captcha" type="text" name="captcha" value="" {}
                    input type="hidden" name="csrf" value="" {}
                    input type="submit" value="Submit" {}
                }
            }

            h2 { "Recent Articles" }
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
            h1 {"Follow my recent development adventures"}
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
