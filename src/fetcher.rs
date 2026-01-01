use anyhow::{anyhow, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::program_pack::Pack;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::{RpcFilterType, Memcmp, MemcmpEncodedBytes};
use spl_token::state::Account as TokenAccount;
use std::str::FromStr;
use crate::types::TokenHolder;

/// Fetch all token holders for a given mint address
pub async fn fetch_token_holders(
    rpc_client: &RpcClient,
    mint_address: &str,
) -> Result<Vec<TokenHolder>> {
    // Parse mint address
    let mint_pubkey = Pubkey::from_str(mint_address)
        .map_err(|e| anyhow!("Invalid mint address: {}", e))?;
    
    // Get mint info to retrieve decimals
    let decimals = get_mint_decimals(rpc_client, &mint_pubkey)?;
    
    // Fetch all token accounts for this mint
    let token_accounts = get_token_accounts_by_mint(rpc_client, &mint_pubkey)?;
    
    // Parse accounts and extract holder information
    let mut holders = Vec::new();
    
    for (pubkey, account) in token_accounts {
        match parse_token_account(&account.data) {
            Ok(token_account) => {
                // Only include accounts with non-zero balance
                if token_account.amount > 0 {
                    holders.push(TokenHolder::new(
                        token_account.owner.to_string(),
                        token_account.amount,
                        decimals,
                    ));
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to parse token account {}: {}", pubkey, e);
            }
        }
    }
    
    Ok(holders)
}

/// Get the decimals for a mint
fn get_mint_decimals(rpc_client: &RpcClient, mint_pubkey: &Pubkey) -> Result<u8> {
    let account = rpc_client.get_account(mint_pubkey)
        .map_err(|e| anyhow!("Failed to fetch mint account: {}", e))?;
    
    let mint = spl_token::state::Mint::unpack_from_slice(&account.data)
        .map_err(|e| anyhow!("Failed to unpack mint data: {}", e))?;
    
    Ok(mint.decimals)
}

/// Fetch all token accounts for a specific mint
fn get_token_accounts_by_mint(
    rpc_client: &RpcClient,
    mint_pubkey: &Pubkey,
) -> Result<Vec<(Pubkey, solana_sdk::account::Account)>> {
    let config = RpcProgramAccountsConfig {
        filters: Some(vec![
            // Filter by token account size (165 bytes)
            RpcFilterType::DataSize(165),
            // Filter by mint address (offset 0, 32 bytes)
            RpcFilterType::Memcmp(Memcmp::new(
                0,
                MemcmpEncodedBytes::Bytes(mint_pubkey.to_bytes().to_vec()),
            )),
        ]),
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            ..RpcAccountInfoConfig::default()
        },
        ..RpcProgramAccountsConfig::default()
    };
    
    let accounts = rpc_client
        .get_program_accounts_with_config(&spl_token::id(), config)
        .map_err(|e| anyhow!("Failed to fetch token accounts: {}", e))?;
    
    Ok(accounts)
}

/// Parse raw account data into a TokenAccount
fn parse_token_account(data: &[u8]) -> Result<TokenAccount> {
    TokenAccount::unpack_from_slice(data)
        .map_err(|e| anyhow!("Failed to unpack token account: {}", e))
}