// Run with: cargo run --bin custom_error
//
// A "thiserror-style" error enum: the error type is just an enum, and a
// derive macro writes the boilerplate (Display/Error impls) for us.

use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
enum ConfigError {
    #[error("config file not found")]
    NotFound,
    #[error("config file is empty")]
    Empty,
    // #[from] means: `?` on an std::io::Error auto-wraps it into this variant.
    #[error("unable to read config file")]
    Io(#[from] std::io::Error),
}

fn load_config() -> Result<String, ConfigError> {
    let path = Path::new("config.json");
    if !path.exists() {
        return Err(ConfigError::NotFound);
    }
    let text = std::fs::read_to_string(path)?; // io::Error converts via #[from]
    if text.trim().is_empty() {
        return Err(ConfigError::Empty);
    }
    Ok(text)
}

fn main() {
    match load_config() {
        Ok(text) => println!("Config loaded: {text}"),
        Err(e) => println!("Error: {e}"),
    }
}
