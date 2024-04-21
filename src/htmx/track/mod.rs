use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json,
};

use crate::{db, AppState};

use super::HtmxComponent;

#[derive(serde::Deserialize, Debug)]
pub struct Interaction {
    action: String,
}

pub struct Track;
impl HtmxComponent<AppState> for Track {
    fn path() -> &'static str {
        "/interact"
    }
    fn handle() -> axum::routing::MethodRouter<AppState> {
        post(
            |State(state): State<AppState>, Json(interaction): Json<Interaction>| async move {
                _ = db::inc(&state.db_pool, &interaction.action).await;
                "".into_response()
            },
        )
    }
}
