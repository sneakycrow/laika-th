pub const DEFAULT_PORT: &str = "3000"; // This is stored as a string to match environment vars

/// The running API configuration
pub struct Config {
    pub port: String,
}

impl Config {
    /// Creates a new config with values from environment variables, falls back to default values
    pub fn from_env() -> Self {
        // Check for core networking such as PORT and HOST
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| DEFAULT_PORT.to_string())
            .parse()
            .expect("PORT must be a number");
        Config { port }
    }
    /// Formats the host and port into an address for a TCPListener to bind to
    pub fn get_address(&self) -> String {
        format!("localhost:{}", &self.port)
    }
}
