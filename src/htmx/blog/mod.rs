use super::HtmxComponent;
use crate::{db, AppState};
use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, MethodRouter},
};
use maud::html;

pub struct BlogContent;
impl HtmxComponent<AppState> for BlogContent {
    fn path() -> &'static str {
        "/blog"
    }

    fn css() -> &'static str {
        include_str!("style.css")
    }

    fn handle() -> MethodRouter<AppState> {
        get(|State(state): State<AppState>| async move {
            _ = db::inc(&state.db_pool, "blog").await;
            html!(
                h1 {"Follow my recent development adventures"}
                hr{}
                div hx-get="/htmx/articles/3/0" hx-trigger="load" {
                    div class="loading-spinner" src="static/images/spinner.svg" {}
                }
            )
            .into_response()
        })
    }
}
