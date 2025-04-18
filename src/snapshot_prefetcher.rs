use crate::config::AppConfig;
use anyhow::{anyhow, Result};
use futures_util::StreamExt; //  Fix for .next()
use reqwest::Client;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{Read, Write}; //  Fix for write_all()
use std::path::Path;

pub async fn run(config: &AppConfig) -> Result<()> {
    let url = config.snapshot_url.trim();

    if url.is_empty() {
        println!(" Skipping snapshot download (snapshot_url is empty)");
        return Ok(());
    }

    let snapshot_path = "./snapshots/snapshot-latest.tar.zst";

    if url.starts_with("file://") {
        let local_path = url.trim_start_matches("file://");
        fs::create_dir_all("./snapshots")?;
        fs::copy(local_path, snapshot_path)?;
        println!(" Copied local snapshot from: {}", local_path);
    } else {
        println!(" Downloading snapshot from: {}", url);

        let client = Client::new();
        let response = match client.get(url).send().await {
            Ok(resp) => {
                if !resp.status().is_success() {
                    return Err(anyhow!(
                        "Snapshot download failed with status: {}",
                        resp.status()
                    ));
                }
                resp
            }
            Err(e) => {
                return Err(anyhow!(" Snapshot fetch failed: {}", e));
            }
        };

        fs::create_dir_all("./snapshots")?;
        let mut file = File::create(snapshot_path)?;
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let bytes = chunk?;
            file.write_all(&bytes)?;
        }

        println!(" Snapshot downloaded to {}", snapshot_path);
    }

    if let Some(expected_hash) = &config.snapshot_sha256 {
        if !expected_hash.trim().is_empty() {
            println!(" Validating snapshot SHA256...");
            let actual_hash = compute_sha256(snapshot_path)?;
            if actual_hash != expected_hash.trim() {
                return Err(anyhow!(
                    " SHA256 mismatch! Expected: {}, Got: {}",
                    expected_hash,
                    actual_hash
                ));
            }
            println!(" SHA256 verified.");
        }
    }

    Ok(())
}

fn compute_sha256<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 4096];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

