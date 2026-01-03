mod config;
mod client;
mod fetcher;
mod filter;
mod types;
mod wallet_age;  
mod bot;
mod whale;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    bot::run_bot().await;
    Ok(())
}