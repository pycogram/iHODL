use std::env;

/// Initialize environment variables from .env file
pub fn init() {
    dotenv::dotenv().ok();
}

pub const MINIMUM_UI_AMOUNT: f64 = 2_000_000.0;
pub const MAX_WALLET_AGE_HOURS: u64 = 48;
pub const MINIMUM_SOL_FOR_WHALE: f64 = 40.0;

/// Get the Solana RPC URL from environment or use default
pub fn get_rpc_url() -> String {
    env::var("SOLANA_RPC_URL")
        .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string())
}

/// Get the commitment level for RPC calls
pub fn get_commitment() -> solana_sdk::commitment_config::CommitmentConfig {
    solana_sdk::commitment_config::CommitmentConfig::confirmed()
}

/// Get Telegram bot token from environment
pub fn telegram_bot_token() -> String {
    env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN not found in .env")
}