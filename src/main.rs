mod data_structs;
mod ds_commands;

use ::serenity::all::Ready;
use ::serenity::all::ResumedEvent;
use data_structs::Data;
use ds_commands::{age, poker};
use poise::serenity_prelude as serenity;
use serenity::async_trait;
use serenity::prelude::*;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Bot connexted as: {}", ready.user.name);
    }
    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Bot resumed");
    }
}
#[tokio::main]
async fn main() {
    println!("Start bot");
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents: ::serenity::prelude::GatewayIntents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), poker()], // Add the poker command
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
