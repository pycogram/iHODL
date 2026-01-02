use serde::{Deserialize, Serialize};

/// Represents a token holder with their balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenHolder {
    pub owner: String,
    pub balance: u64,
    pub decimals: u8,
}

impl TokenHolder {
    pub fn new(owner: String, balance: u64, decimals: u8) -> Self {
        Self {
            owner,
            balance,
            decimals,
        }
    }
    
    /// Get the actual balance as a float considering decimals
    pub fn get_ui_amount(&self) -> f64 {
        self.balance as f64 / 10_f64.powi(self.decimals as i32)
    }
}