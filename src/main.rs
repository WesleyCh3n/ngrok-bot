use reqwest::Client;
use serde::Deserialize;
use std::env;
use teloxide::{prelude::*, utils::command::BotCommands};

const NGROK_API_URL: &str = "http://127.0.0.1:4040/api/tunnels";

#[derive(Debug, Deserialize)]
struct NgrokTunnel {
    proto: String,
    public_url: String,
}

#[derive(Debug, Deserialize)]
struct NgrokResponse {
    tunnels: Vec<NgrokTunnel>,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
enum Command {
    #[command(description = "Get current ngrok TCP port")]
    Getport,
    #[command(description = "Display this help text.")]
    Help,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting ngrok port Telegram bot...");

    let bot = Bot::from_env();
    Command::repl(bot, handle_command).await;
}

async fn handle_command(
    bot: Bot,
    msg: Message,
    cmd: Command,
) -> ResponseResult<()> {
    if let Some(user) = msg.from {
        if user.id.to_string() != env::var("USER_TOKEN").unwrap() {
            bot.send_message(msg.chat.id, "Bye").await?;
            return Ok(());
        }
    }

    match cmd {
        Command::Getport => match get_ngrok_tcp_port().await {
            Some(port) => {
                bot.send_message(msg.chat.id, format!("{}", port)).await?;
            }
            None => {
                bot.send_message(msg.chat.id, "Failed to retrieve").await?;
            }
        },
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
    }

    Ok(())
}

async fn get_ngrok_tcp_port() -> Option<u16> {
    let client = Client::new();
    let response = client.get(NGROK_API_URL).send().await.ok()?;
    let data: NgrokResponse = response.json().await.ok()?;

    for tunnel in data.tunnels {
        if tunnel.proto == "tcp" && tunnel.public_url.starts_with("tcp://") {
            let parts: Vec<&str> = tunnel.public_url.split(':').collect();
            if parts.len() == 3 {
                return parts[2].parse::<u16>().ok();
            }
        }
    }
    None
}
