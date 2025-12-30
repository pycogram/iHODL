// src/config.rs
use std::env;
use once_cell::sync::Lazy;

/// Load environment variables once at runtime
pub static DEV_WALLET: Lazy<String> = Lazy::new(|| {
    env::var("DEV_WALLET").expect("DEV_WALLET must be set in .env")
});

pub static DEV_PRIVATE_KEY: Lazy<String> = Lazy::new(|| {
    env::var("DEV_PRIVATE_KEY").expect("DEV_PRIVATE_KEY must be set in .env")
});

pub static TELEGRAM_BOT_TOKEN: Lazy<String> = Lazy::new(|| {
    env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set in .env")
});

pub static TELEGRAM_CHAT_ID: Lazy<i64> = Lazy::new(|| {
    env::var("TELEGRAM_CHAT_ID")
        .expect("TELEGRAM_CHAT_ID must be set")
        .trim()
        .parse()
        .expect("TELEGRAM_CHAT_ID must be a number")
});

// Bot configuration constants
pub const TOKEN_THRESHOLD: u64 = 100_000; // minimum tokens to be eligible
pub const CYCLE_HOURS: u64 = 1;           // reward cycle interval
pub const WINNER_PERCENTAGE: f64 = 0.10;  // 10% of eligible holders win
pub const MAX_PREV_WINNER_RATIO: f64 = 0.3; // max 30% previous winners allowed
pub const FEE_SPLIT_DEV: f64 = 0.10;      // 10% to dev
pub const FEE_SPLIT_WINNERS: f64 = 0.90;  // 90% to winners
pub const WINNERS_JSON_PATH: &str = "./winners.json";
pub const TOKEN_MINT_ADDRESS: &str = "cfp3w235jf6gceq3zd9rvwj6ad6yhd61yjag8u6wpump";
pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;