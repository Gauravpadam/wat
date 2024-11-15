use anyhow::Context as _;
use serenity::all::CreateAttachment;
use serenity::all::CreateMessage;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use std::path::Path;
use std::process::Command;
use tracing::{error, info};

fn get_image_path() -> &'static Path {
    Path::new("assets/wat.png")
}

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        // Open the file asynchronously
        let image_path = get_image_path();
        if !image_path.exists() {
            eprintln!("Image not found: {:?}", image_path);
            return;
        }
        let file = tokio::fs::File::open(image_path).await;

        match file {
            Ok(f) => {
                // Create a message with the file
                let m = CreateMessage::new()
                    .add_file(CreateAttachment::file(&f, "wat.png").await.unwrap());

                // Check if the message content matches "/wat"
                if msg.content == "/wat" {
                    // Send the message with the file
                    if let Err(why) = msg.channel_id.send_message(&ctx.http, m).await {
                        eprintln!("Error sending message: {why:?}");
                    }
                }
            }
            Err(why) => {
                eprintln!("Error opening file: {why:?}");
            }
        }
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    // Run the `ls` command
    let output = Command::new("ls")
        .output()
        .expect("Failed to execute command");

    // Convert and print the command output
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", stderr);
    }

    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
