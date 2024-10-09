pub mod routes;
mod storage;

use uuid::Uuid;

/// The different errors that can happen when processing a game
pub enum GameError {
    MaxMovesMade,
    MaxPlayersReached,
}

/// A Game of TicTacToe (other games tbd)
struct Game {
    /// The unique ID of a game
    id: Uuid,
    /// The moves made in a given game
    /// The maximum amount of moves that can be made in TicTacToe is 9 (9 cells)
    moves: Vec<Move>,
    /// The maximum amount of players for TicTacToe is 2 (x's and o's)
    // TODO: If other game types are supported, make this dynamic
    players: Vec<Player>,
    storage: storage::Backend,
}

impl Game {
    // TODO: Add delete_player
    /// Creates a new instance, shorthand for Self::default()
    fn new() -> Self {
        Self::default()
    }
    /// Creates a default empty game with random ID
    fn default() -> Self {
        let id = Uuid::new_v4();
        Game {
            id,
            moves: vec![],
            players: vec![],
            storage: storage::Backend::LocalJson,
        }
    }
    /// Save a game to storage
    /// If a game doesn't already exist for the ID with the backend, one will be created
    // TODO: Move creation functionality to its own function and error if game doesn't exist in this call
    async fn save_game(&self, backend: storage::Backend) -> &Self {
        // TODO: Save the game
        self
    }
    /// Append a new move to a game
    /// Checks to see if the maximum moves have been made before committing
    // TODO: Check if the player making the move is in our game
    fn make_move(&mut self, new_move: Move) -> Result<&Self, GameError> {
        const MAX_MOVES: usize = 9;
        // Check if we're at our limit
        if self.moves.len() >= MAX_MOVES {
            return Err(GameError::MaxMovesMade);
        }
        self.moves.push(new_move);
        Ok(self)
    }
    /// Add a player to a game
    /// Checks to see if the maximum players have already been added before committing
    // TODO: If a human player, we should make sure they're not already in the game
    fn add_player(&mut self, player: Player) -> Result<&Self, GameError> {
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
struct Move {
    player: Player,
    position: u32, // Maps to the cell that the move occurred, 1 - 9
}

pub type PlayerID = u32;
/// A player in a game, which can be represented by a human player or the computer
pub enum Player {
    Computer,         // Non-human player
    Player(PlayerID), // Human player with unique (to this game) id
}
