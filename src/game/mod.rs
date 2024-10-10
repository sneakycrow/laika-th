pub mod routes;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The different errors that can happen when processing a game
#[derive(Serialize, Debug)]
pub enum GameError {
    MaxMovesMade,
    MaxPlayersReached,
    FailedToSaveGame,
}

/// A Game of TicTacToe (other games tbd)
#[derive(Serialize)]
struct Game {
    /// The unique ID of a game
    id: Uuid,
    /// The moves made in a given game
    /// The maximum amount of moves that can be made in TicTacToe is 9 (9 cells)
    moves: Vec<Move>,
    /// The maximum amount of players for TicTacToe is 2 (x's and o's)
    // TODO: If other game types are supported, make this dynamic
    players: Vec<Player>,
}

impl Game {
    // TODO: Add delete_player
    /// Creates a new locally saved game
    pub fn new_local() -> Self {
        let id = Uuid::new_v4();
        Game {
            id,
            moves: vec![],
            players: vec![],
        }
    }
    /// Save a game to storage
    /// If a game doesn't already exist for the ID with the backend, one will be created
    pub async fn save_game(&self, storage_prefix: String) -> Result<&Self, GameError> {
        // Make sure our storage path exists first
        std::fs::create_dir_all(&storage_prefix).map_err(|_| GameError::FailedToSaveGame)?;
        // Our file will be {id}.json, prepended with the storage prefix
        let full_path = format!("{}/{}.json", storage_prefix, self.id);
        // Serialize our game
        let json = serde_json::to_string(self).map_err(|_| GameError::FailedToSaveGame)?;
        // Save the serialized game to storage
        std::fs::write(full_path, json).map_err(|_| GameError::FailedToSaveGame)?;
        Ok(self)
    }
    /// Append a new move to a game
    /// Checks to see if the maximum moves have been made before committing
    // TODO: Check if the player making the move is in our game
    fn make_move(mut self, new_move: Move) -> Result<Self, GameError> {
        const MAX_MOVES: usize = 9;
        // Check if we're at our limit
        if self.moves.len() >= MAX_MOVES {
            return Err(GameError::MaxMovesMade);
        }
        self.moves.push(new_move.clone());
        Ok(self)
    }
    /// Add a player to a game
    /// Checks to see if the maximum players have already been added before committing
    // TODO: If a human player, we should make sure they're not already in the game
    fn add_player(mut self, player: Player) -> Result<Self, GameError> {
        // TODO: If extending to other games, make dynamic
        const MAX_PLAYERS: usize = 2;
        // Check if we're at our limit
        if self.players.len() >= MAX_PLAYERS {
            return Err(GameError::MaxPlayersReached);
        }
        self.players.push(player);
        Ok(self)
    }
}

/// A move that a player can make in a game (currently only TicTacToe supported)
#[derive(Deserialize, Serialize, Clone)]
struct Move {
    player: Player, // The player that made the move
    position: u32,  // Maps to the cell that the move occurred, 1 - 9
    turn: u32,      // The turn a move was made
}

pub type PlayerID = String;
#[derive(Deserialize, Serialize, Clone)]
/// A player in a game, which can be represented by a human player or the computer
pub enum Player {
    Computer,         // Non-human player
    Player(PlayerID), // Human player with unique (to this game) id
}
