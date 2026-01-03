# 🧠 Solana Token Holders Analyzer Bot (TokyScanner)

A Telegram bot that analyzes **Solana token holders** and highlights **bundle wallets**, **whales**, and **top holders** in real time.

Built in **Rust** using **Teloxide** and the **Solana RPC**, this bot is designed for speed, clarity, and on-chain transparency.

---

## 🚀 Features

- 🔍 Fetches all token holders for a given mint address
- ✅ Filters holders by minimum token balance
- 🆕 Detects **bundle wallets** (recently created wallets)
- 🐋 Detects **whales** (wallets holding large SOL balances)
- 📊 Calculates bundle & whale percentages
- 🏆 Displays **Top 5 holders** ranked by balance
- ⚡ Runs wallet checks **in parallel** for performance
- 💬 Clean **plain-text Telegram output** (no markdown issues)

---

## 🛠️ Tech Stack

- Rust
- Teloxide (Telegram bot framework)
- Solana RPC
- Tokio (async runtime)
- Anyhow (error handling)

---
## 📦 Project Structure

```text
src/
├── bot.rs # Telegram bot logic
├── client.rs # Solana RPC client
├── fetcher.rs # Token holder fetching
├── filter.rs # Sorting & filtering
├── wallet_age.rs # Wallet age detection
├── whale.rs # Whale detection
├── config.rs # App configuration
└── main.rs # Entry point
```
---

## ▶️ Running the Bot

- Build: cargo build --release
- Run: cargo run --release 

---


## 🤖 Telegram Commands

- `/help` — Show available commands
- `/check <MINT_ADDRESS>` — Analyze token holders

**Example**:
/check So11111111111111111111111111111111111111112

---

### Example Bot Output

![Telegram bot showing token holder analysis](assets/bot-output.png)
