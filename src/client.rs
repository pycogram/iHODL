use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use crate::config;

/// Create and configure an RPC client for Solana .
pub fn create_rpc_client() -> Result<RpcClient> {
    let rpc_url = config::get_rpc_url();
    let commitment = config::get_commitment();
    
    let client = RpcClient::new_with_commitment(rpc_url, commitment);
    
    Ok(client)
}