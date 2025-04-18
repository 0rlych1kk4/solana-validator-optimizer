use crate::config::AppConfig;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use lru::LruCache;
use std::num::NonZeroUsize;
use tokio::time::{Duration, Instant};

#[derive(Debug)]
struct CachedResponse {
    value: String,
    expires_at: Instant,
}

type SharedCache = Arc<Mutex<LruCache<String, CachedResponse>>>;

pub async fn start_rpc_cache(config: &AppConfig) -> anyhow::Result<()> {
    println!(" Starting RPC LRU cache (size: {})", config.cache_size);

    let cache: SharedCache = Arc::new(Mutex::new(LruCache::new(
        NonZeroUsize::new(config.cache_size).unwrap(),
    )));

    // Spawn fake RPC requests to demonstrate caching
    let cache_clone = cache.clone();
    tokio::spawn(async move {
        loop {
            let keys = vec!["getBalance:Orly".to_string(), "getAccountInfo:Wallet123".to_string()];
            for key in keys {
                let result = handle_rpc_request(cache_clone.clone(), key.clone()).await;
                println!("→ Response: {}", result);
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    Ok(())
}

async fn handle_rpc_request(cache: SharedCache, key: String) -> String {
    let mut cache_lock = cache.lock().await;

    // Check cache
    if let Some(entry) = cache_lock.get(&key) {
        if Instant::now() < entry.expires_at {
            println!(" Cache hit for {}", key);
            return format!("(cached) {}", entry.value);
        } else {
            println!(" Cache expired for {}", key);
        }
    } else {
        println!(" Cache miss for {}", key);
    }

    // Simulate fetching fresh data (e.g., via real RPC later)
    let fresh_value = simulate_rpc_fetch(&key).await;

    // Cache for 10 seconds
    let new_entry = CachedResponse {
        value: fresh_value.clone(),
        expires_at: Instant::now() + Duration::from_secs(10),
    };
    cache_lock.put(key.clone(), new_entry);

    fresh_value
}

async fn simulate_rpc_fetch(key: &str) -> String {
    // Simulate network delay
    tokio::time::sleep(Duration::from_millis(200)).await;
    format!("FreshData({})", key)
}

