## $iHODL Reward System 

🚀 Automated reward distribution system for Solana token holders.

This Rust-based script selects eligible $iHODL token holders, distributes rewards from collected fees, and notifies winners via Telegram every 24 hours. Previous winners can win again, and the distribution logic ensures fair randomness while rewarding long-term holders.

*** Ca ***: - 

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
│   ├── config.rs          // All settings here
│   ├── holders.rs
│   ├── rewards.rs
│   ├── winners.rs
│   ├── telegram.rs
│   └── utils.rs
├── winners.json
```
---

```mermaid
flowchart TD
    A[Start Bot / Load .env & Config] --> B[Fetch Holders from Solana RPC]
    B --> C{Filter by TOKEN_THRESHOLD}
    C --> D[Shuffle Holders & Select Winners]
    D --> E[Limit Previous Winners\n(MAX_PREV_WINNER_RATIO)]
    E --> F[Fetch Total Fees\n(fetch_total_fees)]
    F --> G[Split Fees & Send SOL\n- DEV Fee\n- Winners Fee\n- Handle Remainder]
    G --> H[Update winners.json\n(Async / Mutex)]
    H --> I[Notify Winners via Telegram Bot]
    I --> J[Sleep for CYCLE_HOURS then Repeat Loop]

```
---


