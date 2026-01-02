use teloxide::{prelude::*, utils::command::BotCommands};
use anyhow::Result;
use crate::{client, fetcher, filter, config};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Supported commands:")]
enum Command {
    #[command(description = "Display help")]
    Help,
    #[command(description = "Check token holders: /check <MINT_ADDRESS>")]
    Check(String),
}

pub async fn run_bot() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    log::info!("Starting Telegram bot...");

    let bot = Bot::new(config::telegram_bot_token());

    log::info!("Bot started successfully!");

    Command::repl(bot, answer).await;
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .reply_to_message_id(msg.id)
                .await?;
        }
        Command::Check(mint_address) => {
            if mint_address.trim().is_empty() {
                bot.send_message(
                    msg.chat.id,
                    "❌ Please provide a mint address!\n\nUsage: /check <MINT_ADDRESS>",
                )
                .reply_to_message_id(msg.id)
                .await?;
                return Ok(());
            }

            // Show a temporary "processing" message
            let processing_msg = bot
                .send_message(msg.chat.id, "🔍 Fetching token holders... Please wait...")
                .reply_to_message_id(msg.id)
                .await?;

            // Fetch holders
            match fetch_holders(&mint_address).await {
                Ok(response) => {
                    bot.delete_message(msg.chat.id, processing_msg.id).await.ok();
                    bot.send_message(msg.chat.id, response)
                        .reply_to_message_id(msg.id)
                        .await?;
                }
                Err(e) => {
                    bot.delete_message(msg.chat.id, processing_msg.id).await.ok();
                    bot.send_message(msg.chat.id, format!("❌ Error: {}", e))
                        .reply_to_message_id(msg.id)
                        .await?;
                }
            }
        }
    }

    Ok(())
}

async fn fetch_holders(mint_address: &str) -> Result<String> {
    let minimum_ui_amount = config::MINIMUM_UI_AMOUNT;

    // Initialize RPC client
    let rpc_client = client::create_rpc_client()?;

    // Fetch token holders
    let all_holders = fetcher::fetch_token_holders(&rpc_client, mint_address).await?;

    // Filter by minimum UI amount
    let filtered_holders = if minimum_ui_amount > 0.0 {
        filter::filter_by_minimum_ui_amount(all_holders.clone(), minimum_ui_amount)
    } else {
        all_holders.clone()
    };

    // Sort by balance (highest first)
    let sorted_holders = filter::sort_by_balance_desc(filtered_holders);

    // Header
    let mut response = format!(
        "🎯 Token Holders Report\nTotal holders: {} | With ≥{} tokens: {}\n\nTop 5 Holders:\n",
        all_holders.len(),
        format_number(minimum_ui_amount),
        sorted_holders.len()
    );

    if sorted_holders.is_empty() {
        response.push_str("No holders found with the minimum balance.\n");
        return Ok(response);
    }

    // Find max length of token amounts for alignment
    let max_amount_len = sorted_holders
        .iter()
        .take(5)
        .map(|h| format_number(h.get_ui_amount()).len())
        .max()
        .unwrap_or(0);

    // Show top 5 holders
    for (index, holder) in sorted_holders.iter().take(5).enumerate() {
        let amount = format_number(holder.get_ui_amount());
        let padding = " ".repeat(max_amount_len - amount.len());
        response.push_str(&format!(
            "{}. {} -- {}{}\n",
            index + 1,
            truncate_address(&holder.owner),
            padding,
            amount
        ));
    }

    // Show remaining count if any
    if sorted_holders.len() > 5 {
        response.push_str(&format!("\n… and {} more holders", sorted_holders.len() - 5));
    }

    Ok(response)
}

fn format_number(num: f64) -> String {
    if num >= 1_000_000.0 {
        format!("{:.2}M", num / 1_000_000.0)
    } else if num >= 1_000.0 {
        format!("{:.2}K", num / 1_000.0)
    } else {
        format!("{:.2}", num)
    }
}

fn truncate_address(address: &str) -> String {
    if address.len() > 12 {
        format!("{}...{}", &address[..6], &address[address.len() - 6..])
    } else {
        address.to_string()
    }
}
