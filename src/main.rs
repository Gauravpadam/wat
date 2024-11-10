mod utils;
use serenity::builder::{CreateMessage, CreateAttachment};
use serenity::{all::{Context, EventHandler, GatewayIntents, Message}, async_trait, Client};
use utils::conf;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Open the file asynchronously
        let file = tokio::fs::File::open("wat.png").await;
        
        match file {
            Ok(f) => {
                // Create a message with the file
                let m = CreateMessage::new().add_file(CreateAttachment::file(file, "wat.png").await.unwrap());

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

#[tokio::main]
async fn main(){
    let token = &*conf::DISCORD_TOKEN;
    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}


