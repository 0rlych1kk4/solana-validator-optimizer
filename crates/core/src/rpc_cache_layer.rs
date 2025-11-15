use crate::config::AppConfig;
use crate::metrics::{REQUEST_COUNTER, CACHE_HIT_COUNTER, CACHE_MISS_COUNTER}; //  Import all counters
use lru::LruCache;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey; 
use solana_commitment_config::CommitmentConfig;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::Instant;

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

    let rpc_url = config.rpc_url.clone();
    let cache_clone = cache.clone();

    tokio::spawn(async move {
        loop {
            let keys = vec![
                "getBalance:9Vpj7yMy7V7ojAB8BoS5efZLTi3kWJv3bXWQ7vLxB4vG".to_string(),
                "getEpochInfo".to_string(),
            ];

            for key in keys {
                let result = tokio::task::spawn_blocking({
                    let cache = cache_clone.clone();
                    let rpc_url = rpc_url.clone();
                    let key = key.clone();
                    move || {
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        rt.block_on(handle_rpc_request(cache, key, rpc_url))
                    }
                })
                .await
                .unwrap_or_else(|e| format!("Spawn failed: {}", e));

                println!("→ Response: {}", result);
            }

            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    Ok(())
}

async fn handle_rpc_request(
    cache: SharedCache,
    key: String,
    rpc_url: String,
) -> String {
    let mut cache_lock = cache.lock().await;

    if let Some(entry) = cache_lock.get(&key) {
        if Instant::now() < entry.expires_at {
            println!(" Cache hit for {}", key);
            CACHE_HIT_COUNTER.inc(); //  Record hit
            return format!("(cached) {}", entry.value);
        } else {
            println!(" Cache expired for {}", key);
        }
    } else {
        println!(" Cache miss for {}", key);
        CACHE_MISS_COUNTER.inc(); //  Record miss
    }

    let fresh_value = match fetch_from_solana(&key, &rpc_url).await {
        Ok(val) => val,
        Err(e) => format!("Error: {}", e),
    };

    let new_entry = CachedResponse {
        value: fresh_value.clone(),
        expires_at: Instant::now() + Duration::from_secs(10),
    };
    cache_lock.put(key.clone(), new_entry);

    fresh_value
}

async fn fetch_from_solana(key: &str, rpc_url: &str) -> anyhow::Result<String> {
    REQUEST_COUNTER.inc(); //  Track actual RPC request

    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    if key.starts_with("getBalance:") {
        let pubkey_str = key.strip_prefix("getBalance:").unwrap();
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

