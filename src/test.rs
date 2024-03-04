use std::env;

use serenity::all::RoleId;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
        if msg.content == "!user" {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, msg.author.id.to_string())
                .await
            {
                println!("Error sending message: {why:?}");
            }
        }
        if msg.content == "!poker" {
            let role_id = RoleId::new(1214016146559471656);
            let user = &msg.author;
            if user
                .has_role(&ctx, msg.guild_id.unwrap(), role_id)
                .await
                .unwrap_or(false)
            {
                // El usuario tiene el rol específico
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, "Tienes el rol específico.")
                    .await
                {
                    println!("Error enviando el mensaje: {why:?}");
                }
            } else {
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, "No tienes el rol especifico.")
                    .await
                {
                    println!("Error enviando el mensaje {why:?}")
                }
            }
        }
    }
}
#[tokio::main]
async fn main() {
    // Login with a bot token from the environment

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
