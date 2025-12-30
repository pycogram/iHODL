// src/telegram.rs
use teloxide::prelude::*;
use crate::holders::Holder;
use crate::config::{TELEGRAM_BOT_TOKEN, TELEGRAM_CHAT_ID};

/// Maximum Telegram message length
const TELEGRAM_MAX_LEN: usize = 4000;

/// Send Telegram message to announce winners
pub async fn send_telegram_message(winners: &[Holder]) {
    if winners.is_empty() {
        println!("ℹ️ No winners to notify via Telegram.");
        return;
    }

    let bot = Bot::new(TELEGRAM_BOT_TOKEN.clone());

    // Build the message
    let mut message = String::from("🎉 $iHODL Winners 🎉\n\n");
    for (i, winner) in winners.iter().enumerate() {
        message.push_str(&format!("{}. Wallet: {}\n", i + 1, winner.wallet));
        if message.len() > TELEGRAM_MAX_LEN - 100 {
            // Send current chunk
            if let Err(e) = bot.send_message(ChatId(*TELEGRAM_CHAT_ID), message.clone()).await {
                eprintln!("❌ Telegram error: {:?}", e);
            }
            message.clear();
        }
    }

    // Send remaining message
    if !message.is_empty() {
        if let Err(e) = bot.send_message(ChatId(*TELEGRAM_CHAT_ID), message.clone()).await {
            eprintln!("❌ Telegram error: {:?}", e);
        }
    }

    println!("✅ Telegram notification sent for {} winners", winners.len());
}
