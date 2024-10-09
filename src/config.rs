pub const DEFAULT_PORT: &str = "3000";
pub const DEFAULT_STORAGE_PATH: &str = "_gamedata";

/// The running API configuration
pub struct Config {
    pub port: String,
    pub storage_path: String,
}

impl Config {
    /// Creates a new config with values from environment variables, falls back to default values
    pub fn from_env() -> Self {
        // Check for a non-default port
        let port = std::env::var("PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string());
        // Check for a non-default storage path
        let storage_path =
            std::env::var("STORAGE_PATH").unwrap_or_else(|_| DEFAULT_STORAGE_PATH.to_string());
        Config { port, storage_path }
    }
    /// Formats the host and port into an address for a TCPListener to bind to
    pub fn get_address(&self) -> String {
        format!("localhost:{}", &self.port)
    }
}
