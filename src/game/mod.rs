mod storage;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

/// Updates an existing game with additional moves
pub async fn game_handler(Path(game_id): Path<String>) -> impl IntoResponse {
    // TODO: Receive move made from player
    // TODO: Get the Game from storage
    // TODO: Validate and append the move
    // TODO: Make another move as the computer
    // TODO: Return move made to client
    format!("request for game {}", game_id)
}

/// Initializes a new game, with optional included first move
pub async fn start_game(State(state): State<crate::AppState>) -> impl IntoResponse {
    // TODO: Create a game, save it, return the ID
    format!("request for game {} with state", "no_id_yet")
}

/// A Game of TicTacToe (other games tbd)
struct Game {
    /// The unique ID of a game
    id: String,
    /// The moves made in a given game
    /// The maximum amount of moves that can be made in TicTacToe is 9 (9 cells)
    moves: [Move; 9],
    /// The maximum amount of players for TicTacToe is 2 (x's and o's)
    // TODO: If other game types are supported, make this dynamic
    players: [Player; 2],
    storage: storage::Backend,
}

/// A trait for managing a game, such as creating, saving, and updating
trait ManageGame {
    /// Save a game to storage
    async fn save_game(game: &Game, backend: storage::Backend);
}

/// A move that a player can make in a game (currently only TicTacToe supported)
struct Move {
    player: Player,
    position: u32, // Maps to the cell that the move occurred, 1 - 9
}

/// A player in a game, which can be represented by a human player or the computer
pub enum Player {
    Computer,    // Non-human player
    Player(u32), // Player 1, Player 2, etc
}
