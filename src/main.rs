// main.rs - Example usage of Redis client
// Generated automatically by PowerShell setup script

mod redis_client;

use redis_client::{create_redis_client, test_redis_connection};
use std::process;

fn main() {
    println!("Connecting to Redis...");

    // Create Redis client
    let client = match create_redis_client() {
        Ok(client) => {
            println!("✅ Redis client created successfully");
            client
        },
        Err(e) => {
            eprintln!("❌ Failed to create Redis client: {}", e);
            process::exit(1);
        }
    };

    // Test the connection
    if let Err(e) = test_redis_connection(&client) {
        eprintln!("❌ Redis connection test failed: {}", e);
        process::exit(1);
    }

    println!("✅ Successfully connected to Redis!");
    println!("Connection details:");
    println!("- Host: 127.0.0.1");
    println!("- Port: 6379");
    println!("- Authentication: Enabled");
}
