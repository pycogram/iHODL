## $iHODL Reward System 

🚀 Automated reward distribution system for Solana token holders.

This Rust-based script selects eligible $iHODL token holders, distributes rewards from collected fees, and notifies winners via Telegram every 24 hours. Previous winners can win again, and the distribution logic ensures fair randomness while rewarding long-term holders.

Ca: - 

---

## Features 

- Rewards holders with ≥100,000 $iHODL tokens
- Selects holders who have held tokens for ≥24 hours
- Excludes wallets that sold or transferred tokens
- Randomly selects 10% of eligible holders per cycle
- Distributes 10% fees to dev wallet, 90% to winners
- Supports previous winners, with configurable max ratio
- Sends notifications via Telegram bot
- Stores winners in a JSON file (winners.json)

---

## Project Structure

```text
iHODL/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── client.rs          
│   ├── config.rs
│   ├── fetcher.rs
│   ├── filter.rs
│   ├── main.rs
│   └── types.rs
├── winners.json
```



