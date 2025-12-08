// redis_client.rs - Redis connection module for Memurai
// Generated automatically by PowerShell setup script

use redis::Client;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct RedisConfig {
    password: String,
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
struct Config {
    redis: RedisConfig,
}

/// Creates a Redis client using configuration from redis.toml
///
/// # Examples
/// `
/// let client = create_redis_client().expect("Failed to create Redis client");
/// `
pub fn create_redis_client() -> Result<Client, Box<dyn std::error::Error>> {
    // Try to find config file in the project config directory
    let config_path = Path::new("config").join("redis.toml");

    // Read and parse config file
    let config_content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file at {:?}: {}", config_path, e))?;

    let config: Config = toml::from_str(&config_content)
        .map_err(|e| format!("Failed to parse config file: {}", e))?;

    // Create connection URL
    let redis_url = format!(
        "redis://:{}@{}:{}",
        config.redis.password, config.redis.host, config.redis.port
    );

    // Create and return client
    Client::open(redis_url)
        .map_err(|e| format!("Failed to create Redis client: {}", e).into())
}

/// Tests the Redis connection
///
/// # Examples
/// `
/// let client = create_redis_client().expect("Failed to create client");
/// test_redis_connection(&client).expect("Connection test failed");
/// `
pub fn test_redis_connection(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let mut con = client.get_connection()
        .map_err(|e| format!("Failed to get Redis connection: {}", e))?;

    let _: () = redis::cmd("PING")
        .query(&mut con)
        .map_err(|e| format!("Failed to ping Redis server: {}", e))?;

    Ok(())
}
