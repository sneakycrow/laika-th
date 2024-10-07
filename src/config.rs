use std::path::PathBuf;

pub const DEFAULT_PORT: &str = "3000"; // This is stored as a string to match environment vars
pub const DEFAULT_HOST: &str = "0.0.0.0";

/// The running API configuration
pub struct Config {
    pub port: String,
    pub host: String,
    pub web_dir: Option<String>, // Contains a path to a directory to serve
}

impl Config {
    /// Creates a new config with default values
    #[allow(dead_code)]
    pub fn new() -> Self {
        Config {
            port: DEFAULT_PORT.to_string(),
            host: DEFAULT_HOST.to_string(),
            web_dir: None,
        }
    }
    /// Creates a new config with values from environment variables, falls back to default values
    pub fn from_env() -> Self {
        // Check for core networking such as PORT and HOST
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| DEFAULT_PORT.to_string())
            .parse()
            .expect("PORT must be a number");
        let host = std::env::var("HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string());
        // Check whether the EMBED_SPA feature (via environment variable) is set, default to disabled
        let embed_spa = std::env::var("EMBED_SPA")
            .map(|val| val.to_lowercase() == "true")
            .unwrap_or(false);
        let web_dir = if embed_spa {
            let web_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("web/dist");
            Some(web_dir.to_str().unwrap().to_string())
        } else {
            None
        };
        Config {
            port,
            host,
            web_dir,
        }
    }
    /// Formats the host and port into an address for a TCPListener to bind to
    pub fn get_address(&self) -> String {
        format!("{}:{}", &self.host, &self.port)
    }
}
