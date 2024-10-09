use super::Game;

/// Backend integrations for storing data
pub enum Backend {
    // TODO: Extend with Postgres
    LocalJson, // Local .json files
}

// TODO: Replace with actual error
pub type BackendError = String;

trait BackendStorage {
    // TODO: Add delete
    async fn get(id: String) -> Game; // This will get an already existant game
    /// For updating or creating a game. If a game doesn't exist with the ID given, one will be created
    /// TODO: Refactor create to be it's own function and probably error if a game doesn't already exist
    /// NOTE: ^ We should initiate a game with a create function
    async fn save(game: Game) -> Result<Game, BackendError>; // This is for updating AND creating a game
}

impl BackendStorage for Backend {
    async fn get(id: String) -> Game {
        todo!("Implement getting a local json file");
    }

    async fn save(game: Game) -> Result<Game, BackendError> {
        // TODO: Fetch game by ID
        // TODO: Error if json doesn't exist, games should have
        // TODO: Update json if it exists
        todo!("Implement saving a game");
    }
}
