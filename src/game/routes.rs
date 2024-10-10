use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use super::{Game, Move, Player};

#[derive(Deserialize)]
pub struct GameUpdateRequest {
    /// Optional move to start the game with
    move_position: Option<u32>,
    /// Player identifier
    player_id: String,
}

/// Updates an existing game with additional moves
pub async fn update_game(
    Path(game_id): Path<String>,
    State(state): State<Arc<crate::AppState>>,
    Json(payload): Json<GameUpdateRequest>,
) -> impl IntoResponse {
    // First, get our game from the database
    let storage_path = state.config.storage_path.clone();
    let mut game = Game::get(game_id, storage_path.clone())
        .await
        .expect("Could not find game");
    // Next, make the move requested
    let player_move = Move {
        position: payload.move_position.expect("Move position is required"),
        player: Player::Player(payload.player_id),
        turn: game.moves.len() as u32 + 1,
    };
    game.make_move(player_move)
        .expect("Could not make player move");
    // Check if there is a winner after the player's move
    game.check_for_winner();
    // If there's still no winner, make the next move
    if game.winner.is_none() {
        game.make_cpu_move().expect("Could not make computer move");
        // Check if our move won
        game.check_for_winner();
    }
    // Lastly, save the game
    game.save_game(storage_path)
        .await
        .expect("Could not save game");

    Json(game)
}

/// Initializes a new game, with optional included first move
pub async fn start_game(
    State(state): State<Arc<crate::AppState>>,
    Json(payload): Json<GameUpdateRequest>,
) -> impl IntoResponse {
    // Initialize a new game with the given player id as a player at our configured storage path
    let mut game = Game::new_local()
        .add_player(Player::Player(payload.player_id.clone())) // Need to clone here so we can pass again downstream when making move
        .expect("Could not add player to game")
        .add_player(Player::Computer)
        .expect("Could not add computer to game");

    // If a first move was provided, add it to the game
    if let Some(move_position) = payload.move_position {
        // Construct a move from the move position
        // This presumes the provided player_id made the move
        // Since this is the first move, it's considered turn 1
        let player_move = Move {
            position: move_position,
            player: Player::Player(payload.player_id),
            turn: 1,
        };
        game.make_move(player_move)
            .expect("Could not add move to game");
        // Next, have the computer make it's move in response
        game.make_cpu_move().expect("Could not make computer move");
    }
    // Lastly, save the game
    let target_storage_path = state.config.storage_path.clone();
    game.save_game(target_storage_path)
        .await
        .expect("Could not save game");
    Json(game)
}
