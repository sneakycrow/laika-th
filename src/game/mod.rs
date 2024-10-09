mod storage;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

pub async fn game_handler(Path(game_id): Path<String>) -> impl IntoResponse {
    // TODO: Receive move made from player
    // TODO: Get the Game from storage
    // TODO: Validate and append the move
    // TODO: Make another move as the computer
    // TODO: Return move made to client
    format!("request for game {}", game_id)
}

pub async fn start_game(State(state): State<crate::AppState>) -> impl IntoResponse {
    // TODO: Create a game, save it, return the ID
    format!("request for game {} with state", "no_id_yet")
}

struct Game {
    moves: [Move; 9],
    players: [Player; 2],
}

trait ManageGame {
    /// Save a game to storage
    async fn save_game(game: &Game, backend: storage::Backend);
}

struct Move {
    player: Player,
    position: u32, // Maps to the cell that the move occurred, 1 - 9
}

pub enum Player {
    Computer,    // Non-human player
    Player(u32), // Player 1, Player 2, etc
}
