use super::HtmxComponent;
use crate::{db, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use maud::{html, PreEscaped};

pub struct ArticleDetail;
impl HtmxComponent<AppState> for ArticleDetail {
    fn css() -> &'static str {
        include_str!("style.css")
    }

    fn path() -> &'static str {
        "/article/:alias"
    }
    fn handle() -> axum::routing::MethodRouter<AppState> {
        get(
            |Path(alias): Path<String>, State(state): State<AppState>| async move {
                match state
                    .articles
                    .find_by_alias(&alias)
                    .map(|article| article.compiled.as_ref())
                    .flatten()
                {
                    Some(content) => {
                        //track click
                        _ = db::inc(&state.db_pool, alias.as_str()).await;
                        html!(div class="article" {(PreEscaped(content))}).into_response()
                    }
                    None => (StatusCode::NOT_FOUND, "Not found").into_response(),
                }
            },
        )
    }
}
