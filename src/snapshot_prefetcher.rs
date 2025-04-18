use crate::config::AppConfig;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;

pub async fn run(config: &AppConfig) -> anyhow::Result<()> {
    let url = &config.snapshot_url;
    let target_dir = Path::new("snapshots");
    let filename = url.split('/').last().unwrap_or("snapshot.tar.zst");
    let file_path = target_dir.join(filename);

    println!(" Downloading snapshot from: {}", url);

    // Create snapshots/ directory if missing
    if !target_dir.exists() {
        fs::create_dir_all(target_dir)?;
    }

    // Begin downloading
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        anyhow::bail!("Snapshot download failed with status: {}", response.status());
    }

    let mut file = File::create(&file_path).await?;
    let mut stream = response.bytes_stream();
    let mut hasher = Sha256::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        hasher.update(&chunk);
        file.write_all(&chunk).await?;
    }

    file.flush().await?;
    println!(" Snapshot saved to: {:?}", file_path);

    // If SHA256 is set in config, validate it
    if let Some(expected_hash) = &config.snapshot_sha256 {
        let calculated = format!("{:x}", hasher.finalize());
        if calculated != *expected_hash {
            anyhow::bail!(
                " SHA256 mismatch! Expected: {}, Found: {}",
                expected_hash,
                calculated
            );
        }
        println!(" Snapshot SHA256 verified successfully.");
    }

    Ok(())
}

