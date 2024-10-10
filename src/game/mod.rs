pub mod routes;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The different errors that can happen when processing a game
#[derive(Serialize, Debug)]
pub enum GameError {
    MaxMovesMade,
    MaxPlayersReached,
    FailedToSaveGame,
    CouldNotComputeMove,
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
    /// Validates a new move for the given game, returns true if move is allowed
    /// Checks to see if the maximum moves have been made before committing
    fn is_valid_move(&self, new_move: Move) -> Result<Move, GameError> {
        const MAX_MOVES: usize = 9;
        // Check if we're at our limit
        if self.moves.len() >= MAX_MOVES {
            return Err(GameError::MaxMovesMade);
        }
        // Make sure the position is between 1 and 9
        if new_move.position < 1 || new_move.position > 9 {
            return Err(GameError::CouldNotComputeMove);
        }
        // Check if the position is already occupied
        if self.moves.iter().any(|m| m.position == new_move.position) {
            return Err(GameError::CouldNotComputeMove);
        }
        Ok(new_move)
    }
    /// Append a new move to a game
    // TODO: Check if the player making the move is in our game
    fn make_move(&mut self, new_move: Move) -> Result<&Self, GameError> {
        // Only push the move if it's valid
        // This will error if the move isn't valid
        let validated_move = self.is_valid_move(new_move)?;
        self.moves.push(validated_move);
        Ok(self)
    }
    /// Makes a move on the computer behalf
    /// Note: This function _does not_ make your physical cpu move
    fn make_cpu_move(&mut self) -> Result<&Self, GameError> {
        let cpu_move = self.best_next_move()?;
        self.make_move(cpu_move)
    }
    /// Computes the best next move to make in a given game
    fn best_next_move(&self) -> Result<Move, GameError> {
        // Check if there are any winning moves
        for i in 1..9 {
            if self.moves.iter().all(|m| m.position != i as u32) {
                let test_move = Move {
                    player: Player::Computer,
                    position: i,
                    turn: self.moves.len() as u32,
                };
                if self.would_win(&test_move) {
                    return Ok(test_move);
                }
            }
        }
        // Block opponent's winning moves
        for i in 1..9 {
            if self.moves.iter().all(|m| m.position != i as u32) {
                let test_move = Move {
                    player: Player::Player("opponent".to_string()),
                    position: i,
                    turn: self.moves.len() as u32,
                };
                if self.would_win(&test_move) {
                    return Ok(Move {
                        player: Player::Computer,
                        position: i,
                        turn: self.moves.len() as u32,
                    });
                }
            }
        }

        // If no winning or blocking moves, choose a random empty spot
        let empty_spots: Vec<u32> = (0..9)
            .filter(|&i| self.moves.iter().all(|m| m.position != i))
            .collect();

        if !empty_spots.is_empty() {
            let random_index = rand::random::<usize>() % empty_spots.len();
            return Ok(Move {
                player: Player::Computer,
                position: empty_spots[random_index],
                turn: self.moves.len() as u32 + 1,
            });
        }

        Err(GameError::CouldNotComputeMove)
    }

    fn would_win(&self, test_move: &Move) -> bool {
        let winning_combinations = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8], // Rows
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8], // Columns
            [0, 4, 8],
            [2, 4, 6], // Diagonals
        ];

        for combo in winning_combinations.iter() {
            if combo.contains(&(test_move.position as usize))
                && combo.iter().all(|&pos| {
                    self.moves
                        .iter()
                        .any(|m| m.position == pos as u32 && m.player == test_move.player)
                        || pos as u32 == test_move.position
                })
            {
                return true;
            }
        }

        false
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

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Player::Computer, Player::Computer) => true,
            (Player::Player(id1), Player::Player(id2)) => id1 == id2,
            _ => false,
        }
    }
}
