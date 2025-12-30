mod config;
mod holders;
mod rewards;
mod winners;
mod telegram;
mod utils;
mod solana;

use dotenvy::dotenv;
use tokio::time::sleep;
use chrono::Utc;
use std::time::Duration;

use holders::{Holder, fetch_holders};
use winners::{load_winners_json, save_winners_json, limit_previous_winners, SharedWinners};
use rewards::{fetch_total_fees, distribute_fees};
use telegram::send_telegram_message;
use utils::shuffle_vec;

use config::*;

use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("🚀 $iHODL bot starting...");
    println!("💻 Dev wallet: {}", &DEV_WALLET[..5]);

    let winners_json: SharedWinners =
        Arc::new(Mutex::new(load_winners_json(WINNERS_JSON_PATH).await));

    loop {
        println!("⏱ Starting reward cycle...");

        // 1️⃣ Fetch holders
        let mut eligible_holders: Vec<Holder> = fetch_holders().await;
        eligible_holders.retain(|h| h.is_eligible());

        if eligible_holders.is_empty() {
            println!("⚠️ No eligible holders.");
            sleep(Duration::from_secs(CYCLE_HOURS * 3600)).await;
            continue;
        }

        // 2️⃣ Shuffle & select winners
        shuffle_vec(&mut eligible_holders);
        let winner_count =
            (eligible_holders.len() as f64 * WINNER_PERCENTAGE).ceil() as usize;

        let mut winners: Vec<Holder> =
            eligible_holders.into_iter().take(winner_count).collect();

        // 3️⃣ Prevent repeated winners
        {
            let json = winners_json.lock().await;
            winners = limit_previous_winners(winners, &json, MAX_PREV_WINNER_RATIO);
        }

        if winners.is_empty() {
            println!("⚠️ No winners after filtering.");
            sleep(Duration::from_secs(CYCLE_HOURS * 3600)).await;
            continue;
        }

        // 4️⃣ Fetch fees
        let total_fees = fetch_total_fees().await;
        if total_fees == 0 {
            println!("⚠️ No fees collected.");
            sleep(Duration::from_secs(CYCLE_HOURS * 3600)).await;
            continue;
        }

        // 5️⃣ Distribute
        let winner_wallets: Vec<String> =
            winners.iter().map(|h| h.wallet.clone()).collect();

        distribute_fees(total_fees, &winner_wallets).await;

        // 6️⃣ Save winners
        let now = Utc::now();
        {
            let mut json = winners_json.lock().await;
            for winner in &winners {
                json.update_or_add(
                    &winner.wallet,
                    total_fees / winner_wallets.len() as u64,
                    now,
                );
            }
            save_winners_json(WINNERS_JSON_PATH, &json).await;
        }

        // 7️⃣ Notify
        send_telegram_message(&winners).await;

        println!("✅ Cycle complete. Sleeping {} hours...", CYCLE_HOURS);
        sleep(Duration::from_secs(CYCLE_HOURS * 3600)).await;
    }
}
