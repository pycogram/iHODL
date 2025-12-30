// src/winners.rs
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use tokio::fs;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::holders::Holder;

#[derive(Clone, Serialize, Deserialize)]
pub struct WinnerRecord {
    pub wallet: String,
    pub amount_received: u64,
    pub last_won_cycle: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct WinnersJson {
    pub winners: Vec<WinnerRecord>,
}

impl WinnersJson {
    /// Update existing winner or add new record
    pub fn update_or_add(&mut self, wallet: &str, amount: u64, time: DateTime<Utc>) {
        if let Some(w) = self.winners.iter_mut().find(|w| w.wallet == wallet) {
            w.amount_received = w.amount_received.saturating_add(amount);
            w.last_won_cycle = time;
        } else {
            self.winners.push(WinnerRecord {
                wallet: wallet.to_string(),
                amount_received: amount,
                last_won_cycle: time,
            });
        }
    }
}

/// Shared WinnersJson with concurrency safety
pub type SharedWinners = Arc<Mutex<WinnersJson>>;

/// Load winners JSON asynchronously
pub async fn load_winners_json(path: &str) -> WinnersJson {
    match fs::read_to_string(path).await {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => WinnersJson::default(),
    }
}

/// Save winners JSON asynchronously
pub async fn save_winners_json(path: &str, data: &WinnersJson) {
    match serde_json::to_string_pretty(data) {
        Ok(content) => {
            if let Err(e) = fs::write(path, content).await {
                eprintln!("⚠️ Failed to save winners JSON: {}", e);
            }
        }
        Err(e) => eprintln!("⚠️ Failed to serialize winners JSON: {}", e),
    }
}

/// Limit previous winners based on max_ratio
pub fn limit_previous_winners(
    winners: Vec<Holder>,
    history: &WinnersJson,
    max_ratio: f64,
) -> Vec<Holder> {
    if winners.is_empty() {
        return vec![];
    }

    // Identify previous winners
    let mut previous: Vec<_> = winners.iter()
        .filter(|w| history.winners.iter().any(|h| h.wallet == w.wallet))
        .cloned()
        .collect();

    // New winners who haven't won recently
    let mut new_winners: Vec<_> = winners.into_iter()
        .filter(|w| !previous.iter().any(|p| p.wallet == w.wallet))
        .collect();

    // Limit previous winners
    let max_prev = ((previous.len() as f64) * max_ratio).ceil() as usize;
    previous.truncate(max_prev);

    // Combine new winners + allowed previous winners
    new_winners.extend(previous);

    new_winners
}
