use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use super::{Game, Move, Player};

/// Updates an existing game with additional moves
pub async fn update_game(Path(game_id): Path<String>) -> impl IntoResponse {
    // TODO: Receive move made from player
    // TODO: Get the Game from storage
    // TODO: Validate and append the move
    // TODO: Make another move as the computer
    // TODO: Return move made to client
    format!("request for game {}", game_id)
}

#[derive(Deserialize)]
pub struct StartGame {
    /// Optional move to start the game with
    first_move: Option<Move>,
    /// Player identifier
    player_id: String,
}

/// Initializes a new game, with optional included first move
pub async fn start_game(
    State(state): State<Arc<crate::AppState>>,
    Json(payload): Json<StartGame>,
) -> impl IntoResponse {
    // Initialize a new game with the given player id as a player at our configured storage path
    let mut game = Game::new_local()
        .add_player(Player::Player(payload.player_id)) // Need to clone here so we can pass again downstream when making move
        .expect("Could not add player to game")
        .add_player(Player::Computer)
        .expect("Could not add computer to game");

    // If a first move was provided, add it to the game
    // TODO: If the client didn't make the first move, maybe we can?
    if let Some(first_move) = payload.first_move {
        game = game
            .make_move(first_move)
            .expect("Could not add move to game");
    }

    // Lastly, save the game
    let target_storage_path = state.config.storage_path.clone();
    game.save_game(target_storage_path)
        .await
        .expect("Could not save game");
    Json(game)
}
