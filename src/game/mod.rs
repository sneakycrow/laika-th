pub mod routes;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The different errors that can happen when processing a game
#[derive(Serialize, Debug, Deserialize)]
pub enum GameError {
    MaxMovesMade,
    MaxPlayersReached,
    CouldNotFindGame,
    CouldNotComputeMove,
    BadGameData,
}

/// A Game of TicTacToe (other games tbd)
#[derive(Serialize, Deserialize)]
struct Game {
    /// The unique ID of a game
    id: Uuid,
    /// The moves made in a given game
    /// The maximum amount of moves that can be made in TicTacToe is 9 (9 cells)
    moves: Vec<Move>,
    /// The maximum amount of players for TicTacToe is 2 (x's and o's)
    players: Vec<Player>,
    /// The status of the game
    status: GameStatus,
    /// The winner of the game
    winner: Option<Player>,
}

/// The status of a given game
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum GameStatus {
    NotStarted,
    InProgress,
    Complete,
}

/// Maximum players in TicTacToe
const MAX_PLAYERS: usize = 2;
/// Maximum moves in TicTacToe
const MAX_MOVES: usize = 9;

/// The lines representing all possible winning combinations in Tic-Tac-Toe
const TIC_TAC_TOE_LINES: [[u32; 3]; 8] = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9], // Rows
    [1, 4, 7],
    [2, 5, 8],
    [3, 6, 9], // Columns
    [1, 5, 9],
    [3, 5, 7], // Diagonals
];

impl Game {
    // TODO: Add delete_player
    /// Creates a new locally saved game
    pub fn new_local() -> Self {
        let id = Uuid::new_v4();
        Game {
            id,
            moves: vec![],
            players: vec![],
            status: GameStatus::NotStarted,
            winner: None,
        }
    }
    /// Gets an existing game from storage
    pub async fn get(game_id: String, storage_prefix: String) -> Result<Game, GameError> {
        // Read the file contents
        let full_path = format!("{}/{}.json", storage_prefix, game_id);
        let file_contents =
            std::fs::read_to_string(full_path).map_err(|_| GameError::CouldNotFindGame)?;

        // Deserialize the JSON into a Game struct
        let game: Game =
            serde_json::from_str(&file_contents).map_err(|_| GameError::BadGameData)?;

        Ok(game)
    }
    /// Save a game to storage
    /// If a game doesn't already exist for the ID with the backend, one will be created
    pub async fn save_game(&self, storage_prefix: String) -> Result<&Self, GameError> {
        // Make sure our storage path exists first
        std::fs::create_dir_all(&storage_prefix).map_err(|_| GameError::CouldNotFindGame)?;
        // Our file will be {id}.json, prepended with the storage prefix
        let full_path = format!("{}/{}.json", storage_prefix, self.id);
        // Serialize our game
        let json = serde_json::to_string(self).map_err(|_| GameError::CouldNotFindGame)?;
        // Save the serialized game to storage
        std::fs::write(full_path, json).map_err(|_| GameError::CouldNotFindGame)?;
        Ok(self)
    }
    /// Validates a new move for the given game, returns true if move is allowed
    /// Checks to see if the maximum moves have been made before committing
    fn is_valid_move(&self, new_move: Move) -> Result<Move, GameError> {
        // Make sure the game isn't already completed
        if self.status == GameStatus::Complete {
            tracing::debug!("Game already completed");
            return Err(GameError::MaxMovesMade);
        }
        // Check if we're at our move limit
        if self.moves.len() >= MAX_MOVES {
            tracing::debug!("No more moves available");
            return Err(GameError::MaxMovesMade);
        }
        // Make sure the position is between 1 and 9
        if new_move.position < 1 || new_move.position > 9 {
            tracing::debug!("Bad move position provided");
            return Err(GameError::CouldNotComputeMove);
        }
        // Check if the position is already occupied
        if self.moves.iter().any(|m| m.position == new_move.position) {
            tracing::debug!("Position already occupied");
            return Err(GameError::CouldNotComputeMove);
        }
        Ok(new_move)
    }
    /// Validate and append a new move to a game
    // TODO: Check if the player making the move is in our game
    fn make_move(&mut self, new_move: Move) -> Result<&Self, GameError> {
        // Only push the move if it's valid
        // This will error if the move isn't valid
        let validated_move = self.is_valid_move(new_move)?;
        // Before pushing the move, update the game state to in progress if the moves are empty
        if self.moves.len() == 0 {
            self.status = GameStatus::InProgress;
        }
        self.moves.push(validated_move);
        Ok(self)
    }
    /// Makes a move on the computer behalf
    /// Note: This function _does not_ make your physical cpu move
    fn make_cpu_move(&mut self) -> Result<&Self, GameError> {
        let cpu_move = self.best_next_move(Player::Computer)?;
        self.make_move(cpu_move)
    }
    /// Checks for a blocking move
    fn check_blocking_moves(&self, player: Player) -> Option<Move> {
        // Get our opponent
        let opponent = match player {
            Player::Computer => Player::Player(
                self.players
                    .iter()
                    .find_map(|p| {
                        if let Player::Player(id) = p {
                            Some(id.clone())
                        } else {
                            None
                        }
                    })
                    .expect("No human player found"),
            ),
            Player::Player(_) => Player::Computer,
        };
        // See if our opponent has any winning moves coming up
        for line in &TIC_TAC_TOE_LINES {
            // Check for occupied spaces in this line
            let occupied = line
                .iter()
                .filter(|&&pos| {
                    self.moves
                        .iter()
                        .any(|m| m.position == pos && m.player == opponent)
                })
                .count();
            // If there is 2 occupied spaces, we have a blocking move available
            if occupied == 2 {
                if let Some(empty_pos) = line
                    .iter()
                    .find(|&&pos| self.moves.iter().all(|m| m.position != pos))
                {
                    return Some(Move {
                        player,
                        position: *empty_pos,
                        turn: self.moves.len() as u32 + 1,
                    });
                }
            }
        }

        None
    }
    /// Checks for a winning move
    fn check_winning_moves(&self, player: Player) -> Option<Move> {
        // Go through each possible line and see if there's a 3rd cell we can complete it with (winning the game)
        for line in &TIC_TAC_TOE_LINES {
            // Get occupied cells in this line
            let occupied = line
                .iter()
                .filter(|&&pos| {
                    self.moves
                        .iter()
                        .any(|m| m.position == pos && m.player == player)
                })
                .count();
            // See if it has 2/3 of the cells filled out
            if occupied == 2 {
                if let Some(empty_pos) = line
                    .iter()
                    .find(|&&pos| self.moves.iter().all(|m| m.position != pos))
                {
                    return Some(Move {
                        player,
                        position: *empty_pos,
                        turn: self.moves.len() as u32 + 1,
                    });
                }
            }
        }

        None
    }

    /// Computes the best next move to make in a given game
    fn best_next_move(&self, player: Player) -> Result<Move, GameError> {
        // Check if there are any winning moves
        if let Some(winning_move) = self.check_winning_moves(player) {
            return Ok(winning_move);
        }
        // Block opponent's winning moves
        if let Some(blocking_move) = self.check_blocking_moves(Player::Computer) {
            return Ok(blocking_move);
        }
        // If no winning or blocking moves, choose an empty spot
        // Choose the center if it's empty
        if self.moves.iter().all(|m| m.position != 5) {
            return Ok(Move {
                player: Player::Computer,
                position: 5,
                turn: self.moves.len() as u32 + 1,
            });
        }
        // Choose a corner if available
        for corner in [1, 3, 7, 9] {
            if self.moves.iter().all(|m| m.position != corner) {
                return Ok(Move {
                    player: Player::Computer,
                    position: corner,
                    turn: self.moves.len() as u32 + 1,
                });
            }
        }
        // If no corners are available, choose any empty side
        for side in [2, 4, 6, 8] {
            if self.moves.iter().all(|m| m.position != side) {
                return Ok(Move {
                    player: Player::Computer,
                    position: side,
                    turn: self.moves.len() as u32 + 1,
                });
            }
        }
        tracing::error!("Could not find move");
        Err(GameError::CouldNotComputeMove)
    }
    /// Checks to see if the game is complete and declares a winner if one exists
    fn check_for_complete(&mut self) -> &Self {
        // Check if anyone has won or if there's a draw
        match self.has_won() {
            Some(winner) => {
                // Mark the game as complete and declare winner
                self.status = GameStatus::Complete;
                self.winner = Some(winner);
            }
            None => {
                // Check if game has no moves left, if so, presume a draw
                if self.moves.len() >= MAX_MOVES {
                    self.status = GameStatus::Complete;
                }
            }
        };
        self
    }
    /// Calculates if a game has been won by a player
    fn has_won(&self) -> Option<Player> {
        // Go through each line and see if a player has completed it
        for line in TIC_TAC_TOE_LINES.iter() {
            if let Some(first_move) = self.moves.iter().find(|m| m.position == line[0]) {
                if line.iter().all(|&pos| {
                    self.moves
                        .iter()
                        .any(|m| m.position == pos && m.player == first_move.player)
                }) {
                    return Some(first_move.player.clone());
                }
            }
        }
        None
    }
    /// Add a player to a game
    /// Checks to see if the maximum players have already been added before committing
    // TODO: If a human player, we should make sure they're not already in the game
    fn add_player(mut self, player: Player) -> Result<Self, GameError> {
        // TODO: If extending to other games, make dynamic
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
#[derive(Deserialize, Serialize, Clone, Debug)]
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
