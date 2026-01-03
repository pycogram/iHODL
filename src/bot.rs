use teloxide::{prelude::*, utils::command::BotCommands};
use anyhow::Result;
use crate::{client, fetcher, filter, wallet_age, whale, config};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Supported commands:")]
enum Command {
    #[command(description = "Display help")]
    Help,
    #[command(description = "Check token holders: /check <MINT_ADDRESS>")]
    Check(String),
}

pub async fn run_bot() {
    pretty_env_logger::init();
    log::info!("Starting Telegram bot...");

    // Load .env through config
    config::init();
    
    // Get token from config
    let token = config::telegram_bot_token();

    let bot = Bot::new(token);
    
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
            // Validate mint address format
            if mint_address.trim().is_empty() {
                bot.send_message(
                    msg.chat.id,
                    "❌ Please provide a mint address!\n\nUsage: /check <MINT_ADDRESS>"
                )
                .reply_to_message_id(msg.id)
                .await?;
                return Ok(());
            }

            // Send processing message
            let processing_msg = bot
                .send_message(msg.chat.id, "🔍 Analyzing token address... \n\nPlease wait...")
                .reply_to_message_id(msg.id)
                .await?;

            // Fetch and filter holders
            match fetch_holders(&mint_address).await {
                Ok(response) => {
                    // Delete processing message
                    bot.delete_message(msg.chat.id, processing_msg.id).await.ok();
                    
                    // Send result without markdown
                    bot.send_message(msg.chat.id, response)
                        .reply_to_message_id(msg.id)
                        .await?;
                }
                Err(e) => {
                    // Delete processing message
                    bot.delete_message(msg.chat.id, processing_msg.id).await.ok();
                    
                    // Send error
                    bot.send_message(
                        msg.chat.id,
                        format!("❌ Error: {}", e)
                    )
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
    let max_wallet_age_hours = config::MAX_WALLET_AGE_HOURS;
    let minimum_sol_for_whale = config::MINIMUM_SOL_FOR_WHALE;
    
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
    
    // Check for bundle wallets and whales IN PARALLEL for speed
    let mut checks = Vec::new();
    for holder in &filtered_holders {
        let rpc_client = client::create_rpc_client()?;
        let owner = holder.owner.clone();
        let check = tokio::spawn(async move {
            let is_new = wallet_age::is_new_wallet(&rpc_client, &owner, max_wallet_age_hours)
                .unwrap_or(false);
            let is_whale = whale::is_whale(&rpc_client, &owner, minimum_sol_for_whale)
                .unwrap_or(false);
            (is_new, is_whale)
        });
        checks.push((holder.clone(), check));
    }
    
    let mut bundle_wallets: Vec<_> = Vec::new();
    let mut whale_wallets: Vec<_> = Vec::new();
    for (holder, check) in checks {
        if let Ok((is_new, is_whale)) = check.await {
            if is_new {
                bundle_wallets.push(holder.clone());
            }
            if is_whale {
                whale_wallets.push(holder.clone());
            }
        }
    }
    
    // Sort by balance (highest first)
    let sorted_holders = filter::sort_by_balance_desc(filtered_holders.clone());
    
    // Calculate percentages
    let bundle_percentage = if filtered_holders.is_empty() {
        0.0
    } else {
        (bundle_wallets.len() as f64 / filtered_holders.len() as f64) * 100.0
    };
    
    let whale_percentage = if filtered_holders.is_empty() {
        0.0
    } else {
        (whale_wallets.len() as f64 / filtered_holders.len() as f64) * 100.0
    };
    
    // Format response
    let mut response = format!(
        "🎯 Token Holders Report\n\n\
        📊 Total holders: {}\n\
        ✅ Holders with {}+ tokens: {}\n\
        🆕 Bundle wallets (created within {}h): {} ({:.1}%)\n\
        🐋 Whales ({}+ SOL): {} ({:.1}%)\n\n",
        all_holders.len(),
        format_number(minimum_ui_amount),
        filtered_holders.len(),
        max_wallet_age_hours,
        bundle_wallets.len(),
        bundle_percentage,
        minimum_sol_for_whale,
        whale_wallets.len(),
        whale_percentage
    );
    
    if bundle_wallets.is_empty() && whale_wallets.is_empty() {
        response.push_str("✅ No bundle wallets or whales detected!\n\n");
    } else {
        if !bundle_wallets.is_empty() {
            response.push_str(&format!(
                "⚠️ Warning: {:.1}% are bundle wallets\n",
                bundle_percentage
            ));
        }
        if !whale_wallets.is_empty() {
            response.push_str(&format!(
                "🐋 {:.1}% are whales with significant SOL holdings\n",
                whale_percentage
            ));
        }
        response.push_str("\n");
    }
    
    if sorted_holders.is_empty() {
        response.push_str("No holders found with the minimum balance.");
    } else {
        response.push_str("Top 5 Holders:\n");
        response.push_str("━━━━━━━━━━━━━━━━━━━━━━\n");
        
        // Show top 3 holders
        for (index, holder) in sorted_holders.iter().take(3).enumerate() {
            let is_bundle = bundle_wallets.iter().any(|b| b.owner == holder.owner);
            let is_whale = whale_wallets.iter().any(|w| w.owner == holder.owner);
            
            let mut markers = String::new();
            if is_bundle {
                markers.push_str(" 🆕");
            }
            if is_whale {
                markers.push_str(" 🐋");
            }
            
            response.push_str(&format!(
                "{}. {} tokens{}\n   {}\n\n",
                index + 1,
                format_number(holder.get_ui_amount()),
                markers,
                truncate_address(&holder.owner)
            ));
        }
        
        if sorted_holders.len() > 3 {
            response.push_str(&format!("... and {} more holders\n", sorted_holders.len() - 3));
        }
        
        // Legend
        let mut legend = Vec::new();
        if !bundle_wallets.is_empty() {
            legend.push(format!("🆕 = Bundle wallet (created within {}h)", max_wallet_age_hours));
        }
        if !whale_wallets.is_empty() {
            legend.push(format!("🐋 = Whale ({}+ SOL)", minimum_sol_for_whale));
        }
        
        if !legend.is_empty() {
            response.push_str("\n");
            response.push_str(&legend.join("\n"));
        }
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
        format!("{}...{}", &address[..6], &address[address.len()-6..])
    } else {
        address.to_string()
    }
}