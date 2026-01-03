use anyhow::{anyhow, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

/// Check if a wallet was created within the specified hours
pub fn is_new_wallet(
    rpc_client: &RpcClient,
    wallet_address: &str,
    max_age_hours: u64,
) -> Result<bool> {
    let wallet_creation_time = get_wallet_creation_time(rpc_client, wallet_address)?;
    
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| anyhow!("Failed to get current time: {}", e))?
        .as_secs() as i64;
    
    let wallet_age_seconds = current_time - wallet_creation_time;
    let max_age_seconds = (max_age_hours * 3600) as i64;
    
    Ok(wallet_age_seconds <= max_age_seconds)
}

/// Get the timestamp when a wallet was created (first transaction)
pub fn get_wallet_creation_time(
    rpc_client: &RpcClient,
    wallet_address: &str,
) -> Result<i64> {
    let pubkey = Pubkey::from_str(wallet_address)
        .map_err(|e| anyhow!("Invalid wallet address: {}", e))?;
    
    // Get all signatures for this wallet
    let signatures = rpc_client
        .get_signatures_for_address(&pubkey)
        .map_err(|e| anyhow!("Failed to fetch signatures: {}", e))?;
    
    if signatures.is_empty() {
        return Err(anyhow!("No transactions found for wallet"));
    }
    
    // Get the oldest transaction (last in the list)
    let oldest_signature = &signatures[signatures.len() - 1];
    
    oldest_signature.block_time
        .ok_or_else(|| anyhow!("Block time not available"))
}