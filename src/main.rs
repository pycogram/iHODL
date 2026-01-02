mod config;
mod client;
mod fetcher;
mod filter;
mod types;
mod bot;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    bot::run_bot().await;
    Ok(())
}
