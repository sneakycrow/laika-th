use uuid::Uuid;

use super::Game;

// TODO: Replace with actual error
pub type BackendError = String;

/// Backend integrations for storing data
pub enum Backend {
    // TODO: Extend with Postgres
    LocalJson, // Local .json files
}

impl Backend {
    /// Gets a game by given ID
    async fn get(id: Uuid) -> Result<Game, BackendError> {
        todo!("Implement getting a local json file");
    }
    /// Save a game to storage
    /// If a game does not exist in the given storage, one is created
    async fn save(game: Game) -> Result<Game, BackendError> {
        // TODO: Fetch game by ID
        // TODO: Error if json doesn't exist, games should have
        // TODO: Update json if it exists
        todo!("Implement saving a game");
    }
}
