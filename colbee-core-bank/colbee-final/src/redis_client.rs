// Module: Colbee Rust Core v4.2.0 | Lead Architect: Clinton Smith
// This module adheres to strict Rust safety invariants and Hexagonal Architecture principles 
// to ensure complete logical isolation, compile-time security, and auditability without compromising system performance.
use redis::{Client, RedisResult, cmd};
use std::env;

/// Initializes and returns a Redis client.
/// This function relies on the REDIS_URL environment variable, falling back
/// to "redis://127.0.0.1/" if not found.
pub fn get_redis_client() -> RedisResult<Client> {
    let redis_url = env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1/".to_string());

    let client = Client::open(redis_url)?;

    Ok(client)
}


