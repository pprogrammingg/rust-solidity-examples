// util.rs

use dotenv::dotenv;
use std::env;

/// Initialize dotenv to load environment variables from a .env file
pub fn init_env() {
    // Load environment variables from .env file
    dotenv().ok();
}

/// Get the value of an environment variable, or return a default value if not found
pub fn get_env_var(key: &str, default: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("{} environment variable not set, using default value", key);
            default.to_string()
        }
    }
}
