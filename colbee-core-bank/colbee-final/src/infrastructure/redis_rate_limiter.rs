// Module: Colbee Rust Core v4.2.0 | Lead Architect: Clinton Smith
// This module adheres to strict Rust safety invariants and Hexagonal Architecture principles 
// to ensure complete logical isolation, compile-time security, and auditability without compromising system performance.
use redis::AsyncCommands;
use std::time::Duration;

pub async fn is_rate_limited(con: &mut redis::aio::MultiplexedConnection, key: &str) -> bool {
    let limit: u32 = 100;
    let window: u64 = 60;
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let result: u32 = con.zremrangebyscore(format!("rl:{{{} }}", key), 0, now - window).await.unwrap_or(0);
    let count: u32 = con.zcard(format!("rl:{{{} }}", key)).await.unwrap_or(0);
    if count >= limit { return true; }
    con.zadd(format!("rl:{{{} }}", key), now, now).await.ok();
    con.expire(format!("rl:{{{} }}", key), window as usize).await.ok();
    false
}


