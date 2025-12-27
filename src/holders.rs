use chrono::{DateTime, Duration, Utc};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use spl_token::state::Account as TokenAccount;
use solana_program::program_pack::Pack;
use crate::config::{TOKEN_MINT_ADDRESS, TOKEN_THRESHOLD};

/// Represents a token holder
#[derive(Clone, Debug)]
pub struct Holder {
    pub wallet: String,
    pub balance: u64,
    pub first_seen: DateTime<Utc>,
    pub never_sold: bool,
    pub never_transferred: bool,
}

impl Holder {
    /// Returns how long the holder has held the token (in hours)
    pub fn holding_duration_hours(&self) -> i64 {
        (Utc::now() - self.first_seen).num_hours()
    }

    /// Check if holder is eligible
    pub fn is_eligible(&self) -> bool {
        self.balance >= TOKEN_THRESHOLD
    }
}

/// Fetch holders from Solana RPC
pub async fn fetch_holders() -> Vec<Holder> {
    let client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());

    let mint_pubkey = TOKEN_MINT_ADDRESS
        .parse::<Pubkey>()
        .expect("Invalid TOKEN_MINT_ADDRESS");

    // Fetch token accounts by owner using mint filter
    let token_accounts = match client.get_token_accounts_by_owner(
        &mint_pubkey,
        solana_client::rpc_request::TokenAccountsFilter::Mint(mint_pubkey),
    ) {
        Ok(accounts) => accounts,
        Err(e) => {
            eprintln!("Failed to fetch token accounts: {:?}", e);
            return vec![];
        }
    };

    let now = Utc::now();
    let mut holders: Vec<Holder> = Vec::new();

    for ta in token_accounts {
        let wallet = ta.pubkey.to_string();
        let data = &ta.account.data;

        // Try to deserialize the token account
        if let Ok(token_account) = TokenAccount::unpack(data) {
            let balance = token_account.amount;
            if balance >= TOKEN_THRESHOLD {
                holders.push(Holder {
                    wallet,
                    balance,
                    first_seen: now - Duration::hours(48), // TODO: persist first_seen properly
                    never_sold: true,
                    never_transferred: true,
                });
            }
        }
    }

    holders
}
