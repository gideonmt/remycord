use anyhow::Result;
use serenity::prelude::*;
use serenity::model::prelude::*;
use std::sync::Arc;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Connected as: {}", ready.user.name);
    }
}

pub async fn connect_and_verify(token: &str) -> Result<String> {
    let intents = GatewayIntents::GUILDS 
        | GatewayIntents::GUILD_MESSAGES 
        | GatewayIntents::MESSAGE_CONTENT;
    
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await?;
    
    // Get the current user before starting the client
    let http = Arc::clone(&client.http);
    let user = http.get_current_user().await?;
    let username = user.name.clone();
    
    // Start the client in a background task
    tokio::spawn(async move {
        if let Err(why) = client.start().await {
            eprintln!("Client error: {:?}", why);
        }
    });
    
    Ok(username)
}
