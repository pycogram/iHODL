// src/solana.rs
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use crate::config::DEV_PRIVATE_KEY;
use bs58;

/// Connect to Solana RPC (mainnet by default)
pub fn connect() -> RpcClient {
    // Use devnet if needed: "https://api.devnet.solana.com"
    RpcClient::new("https://api.mainnet-beta.solana.com".to_string())
}

/// Load the dev Keypair from base58 private key
pub fn load_dev_keypair() -> Keypair {
    let bytes = bs58::decode(DEV_PRIVATE_KEY.as_str())
        .into_vec()
        .expect("Invalid DEV_PRIVATE_KEY");
    Keypair::from_bytes(&bytes).expect("Failed to parse DEV_PRIVATE_KEY")
}

/// Send SOL (in lamports) from dev wallet to a recipient
pub fn send_sol(client: &RpcClient, recipient: &str, lamports: u64) -> Result<String, Box<dyn std::error::Error>> {
    if lamports == 0 {
        println!("⚠️ Skipping 0 lamports transfer to {}", recipient);
        return Ok("Skipped".to_string());
    }

    let dev_keypair = load_dev_keypair();
    let recipient_pubkey = recipient.parse::<Pubkey>()?;

    let ix = system_instruction::transfer(&dev_keypair.pubkey(), &recipient_pubkey, lamports);
    let latest_blockhash = client.get_latest_blockhash()?;

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&dev_keypair.pubkey()),
        &[&dev_keypair],
        latest_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&tx)?;
    println!("💸 Sent {} lamports to {} | Tx: {}", lamports, recipient, signature);
    Ok(signature.to_string())
}
