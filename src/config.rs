pub const DEFAULT_PORT: &str = "3000"; // This is stored as a string to match environment vars
pub const DEFAULT_HOST: &str = "0.0.0.0";

/// The running API configuration
pub struct Config {
    pub port: String,
    pub host: String,
}

impl Config {
    /// Creates a new config with default values
    pub fn new() -> Self {
        Config {
            port: DEFAULT_PORT.to_string(),
            host: DEFAULT_HOST.to_string(),
        }
    }
    /// Creates a new config with values from environment variables, falls back to default values
    pub fn from_env() -> Self {
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| DEFAULT_PORT.to_string())
            .parse()
            .expect("PORT must be a number");

        let host = std::env::var("HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string());

        Config { port, host }
    }
    /// Formats the host and port into an address for a TCPListener to bind to
    pub fn get_address(&self) -> String {
        format!("{}:{}", &self.host, &self.port)
    }
}
