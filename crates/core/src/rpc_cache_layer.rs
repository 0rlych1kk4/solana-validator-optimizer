use crate::config::AppConfig;
use crate::metrics::{CACHE_HIT_COUNTER, CACHE_MISS_COUNTER, REQUEST_COUNTER};
use lru::LruCache;
use once_cell::sync::Lazy;
use prometheus::IntCounter;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::Instant;

#[derive(Debug, Clone)]
struct CachedResponse {
    value: String,
    expires_at: Instant,
}

type SharedCache = Arc<Mutex<LruCache<String, CachedResponse>>>;

/// Optional but useful: counts actual upstream RPC calls to Solana.
/// This is different from REQUEST_COUNTER which counts requests handled by this layer.
pub static UPSTREAM_REQUEST_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    prometheus::register_int_counter!(
        "rpc_upstream_requests_total",
        "Total number of upstream RPC calls made to Solana"
    )
    .expect("failed to register rpc_upstream_requests_total")
});

/// Starts the background RPC cache loop (spawns a task and returns immediately).
pub async fn start_rpc_cache(config: &AppConfig) -> anyhow::Result<()> {
    println!(" Starting RPC LRU cache (size: {})", config.cache_size);

    let cache: SharedCache = Arc::new(Mutex::new(LruCache::new(
        NonZeroUsize::new(config.cache_size)
            .ok_or_else(|| anyhow::anyhow!("cache_size must be > 0"))?,
    )));

    let rpc_url = config.rpc_url.clone();
    let cache_clone = cache.clone();

    tokio::spawn(async move {
        loop {
            let keys = default_keys();

            for key in keys {
                let result = handle_rpc_request(cache_clone.clone(), key, &rpc_url).await;
                println!("→ Response: {}", result);
            }

            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    Ok(())
}

/// Runs one cache cycle (warm-up) and exits.
/// Intended for: `svo rpc-cache --once` (cronjob/init-container use).
pub async fn run_rpc_cache_once(config: &AppConfig) -> anyhow::Result<()> {
    println!(
        " Running RPC cache once (size: {}, rpc_url: {})",
        config.cache_size, config.rpc_url
    );

    let cache: SharedCache = Arc::new(Mutex::new(LruCache::new(
        NonZeroUsize::new(config.cache_size)
            .ok_or_else(|| anyhow::anyhow!("cache_size must be > 0"))?,
    )));

    let rpc_url = config.rpc_url.clone();
    let keys = default_keys();

    for key in keys {
        let result = handle_rpc_request(cache.clone(), key, &rpc_url).await;
        println!("→ Response: {}", result);
    }

    Ok(())
}

fn default_keys() -> Vec<String> {
    vec![
        "getBalance:9Vpj7yMy7V7ojAB8BoS5efZLTi3kWJv3bXWQ7vLxB4vG".to_string(),
        "getEpochInfo".to_string(),
    ]
}

/// Handles a single request:
/// - increments REQUEST_COUNTER for every request handled (hit or miss)
/// - increments HIT/MISS counters appropriately
/// - fetches from Solana only on miss/expired
async fn handle_rpc_request(cache: SharedCache, key: String, rpc_url: &str) -> String {
    // This is the most intuitive definition: "handled by this layer"
    REQUEST_COUNTER.inc();

    // 1) Fast path: check cache without holding lock during I/O
    {
        let mut cache_lock = cache.lock().await;

        if let Some(entry) = cache_lock.get(&key) {
            if Instant::now() < entry.expires_at {
                println!(" Cache hit for {}", key);
                CACHE_HIT_COUNTER.inc();
                return format!("(cached) {}", entry.value);
            }

            // expired
            println!(" Cache expired for {}", key);
            CACHE_MISS_COUNTER.inc();
        } else {
            println!(" Cache miss for {}", key);
            CACHE_MISS_COUNTER.inc();
        }
    } // lock dropped here

    // 2) Miss path: fetch without holding cache lock
    let fresh_value = match fetch_from_solana(&key, rpc_url).await {
        Ok(val) => val,
        Err(e) => format!("Error: {}", e),
    };

    // 3) Store result back in cache
    {
        let mut cache_lock = cache.lock().await;

        let new_entry = CachedResponse {
            value: fresh_value.clone(),
            expires_at: Instant::now() + Duration::from_secs(10),
        };

        cache_lock.put(key, new_entry);
    }

    fresh_value
}

async fn fetch_from_solana(key: &str, rpc_url: &str) -> anyhow::Result<String> {
    // Counts only real upstream calls to Solana.
    UPSTREAM_REQUEST_COUNTER.inc();

    let client =
        RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    if key.starts_with("getBalance:") {
        let pubkey_str = key
            .strip_prefix("getBalance:")
            .ok_or_else(|| anyhow::anyhow!("invalid getBalance key format"))?;

        let pubkey = pubkey_str.parse::<Pubkey>()?;
        let lamports = client.get_balance(&pubkey).await?;
        Ok(format!("Balance for {}: {} lamports", pubkey_str, lamports))
    } else if key == "getEpochInfo" {
        let epoch_info = client.get_epoch_info().await?;
        Ok(format!(
            "Epoch: {}, Slot: {}, Block height: {}",
            epoch_info.epoch, epoch_info.absolute_slot, epoch_info.block_height
        ))
    } else {
        Ok("Unsupported RPC key".to_string())
    }
}
