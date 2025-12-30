// src/rewards.rs
use crate::solana::{connect, send_sol};
use crate::config::{LAMPORTS_PER_SOL, FEE_SPLIT_DEV, FEE_SPLIT_WINNERS, DEV_WALLET};

/// 1 SOL = 1_000_000_000 lamports
// pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

/// Fetch total collected fees (mocked for now)
pub async fn fetch_total_fees() -> u64 {
    // TODO: Fetch real fees from on-chain program or PumpFun
    100 * LAMPORTS_PER_SOL
}

/// Send SOL to a wallet
pub async fn send_sol_to_wallet(wallet: &str, lamports: u64) {
    let client = connect();

    match send_sol(&client, wallet, lamports) {
        Ok(sig) => println!("✅ Transfer successful: {}", sig),
        Err(e) => eprintln!("❌ Failed to send {} lamports to {}: {:?}", lamports, wallet, e),
    }
}

/// Split and send fees to dev and winners
pub async fn distribute_fees(total_fees: u64, winner_wallets: &[String]) {
    if total_fees == 0 || winner_wallets.is_empty() {
        println!("⚠️ No fees or no winners to distribute.");
        return;
    }

    let dev_fee = (total_fees as f64 * FEE_SPLIT_DEV) as u64;
    let winners_fee = (total_fees as f64 * FEE_SPLIT_WINNERS) as u64;
    let per_winner_fee = winners_fee / winner_wallets.len() as u64;
    let remainder = winners_fee % winner_wallets.len() as u64;

    // Send dev fee
    send_sol_to_wallet(&DEV_WALLET, dev_fee).await;

    // Send to winners
    for (i, wallet) in winner_wallets.iter().enumerate() {
        let amount = if i == 0 { per_winner_fee + remainder } else { per_winner_fee };
        send_sol_to_wallet(wallet, amount).await;
    }
}
