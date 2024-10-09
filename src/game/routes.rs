use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

/// Updates an existing game with additional moves
pub async fn update_game(Path(game_id): Path<String>) -> impl IntoResponse {
    // TODO: Receive move made from player
    // TODO: Get the Game from storage
    // TODO: Validate and append the move
    // TODO: Make another move as the computer
    // TODO: Return move made to client
    format!("request for game {}", game_id)
}

/// Initializes a new game, with optional included first move
pub async fn start_game(State(state): State<Arc<crate::AppState>>) -> impl IntoResponse {
    // TODO: Create a game, save it, return the ID
    format!("request for game {} with state", "no_id_yet")
}
