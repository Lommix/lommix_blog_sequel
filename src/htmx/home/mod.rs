use super::HtmxComponent;
use crate::{db, AppState};
use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, MethodRouter},
};
use maud::html;

pub struct HomeContent;

impl HtmxComponent<AppState> for HomeContent {
    fn path() -> &'static str {
        "/home"
    }

    fn css() -> &'static str {
        include_str!("style.css")
    }

    fn handle() -> MethodRouter<AppState> {
        get(|State(state): State<AppState>| async move {
            _ = db::inc(&state.db_pool, "page visit").await;

            html!(
                h1 { "Welcome! Develop with me!" }

                hr {};

                p {"I am building a game called Panzatier.
                    It's a Top-Down sci-fi Roguelike written in Rust using the Bevy Engine.
                    I have set up a CI Pipline to automatically deploy my current development progress
                    to this blog using Web Assembly. It is very raw and sometimes may be broken,
                    depending on what I am currently working on."}

                wasm-frame track="panzatier-play" cover="wasm/panzatier/cover.jpeg" src="wasm/panzatier/index.html" fullscreen="true" {}

                div class="info"{
                    table class="keybind-table"{
                        tr {
                            th{"Action"}
                            th{"Keybind"}
                        }
                        tr{
                            td{"WASD"}
                            td{"Movement"}
                        }
                        tr{
                            td{"SPACE"}
                            td{"Hold to drift"}
                        }
                        tr{
                            td{"MOUSEWHEEL"}
                            td{"Zoom"}
                        }
                        tr{
                            td{"Q"}
                            td{"Bomb"}
                        }
                    }

                    div {
                        p {"It's still at the beginning stages and I'm working on it when I have spare time. WebGL has its limits and some things like particles and compute shaders don't work in the browser. So this is a simpler version of the game. If you have any thoughts or ideas, drop me a note!"}
                        p {"If you want to see more, checkout my devlogs on youtube! " a target="_blank" href="https://www.youtube.com/watch?v=0csQQaFwD1A" {"My latest video"}}
                    }
                }

                div id="feedback-container"{
                    button class="feedback-button" hx-get="/htmx/feedback" hx-target="#feedback-container" {"You have somthing to say? Give me feedback!"}
                }

                h2 { "Recent Articles" }
                hr{}
                div hx-get="/htmx/articles/3/0" hx-trigger="load" {}
            ).into_response()
        })
    }
}
