use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

pub async fn game_handler(Path(game_id): Path<String>) -> impl IntoResponse {
    format!("request for game {}", game_id)
}

pub async fn start_game(State(state): State<crate::AppState>) -> impl IntoResponse {
    format!("request for game {} with state", "no_id_yet")
}
