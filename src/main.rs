mod data_structs;
mod db_config;
mod discord_commands;
mod discord_handler;
mod table_users;
use data_structs::Data;
use db_config::setup_database;
use discord_commands::{poker, poker_discount, poker_search, poker_top};
use discord_handler::Handler;
use poise::serenity_prelude::{
    client::ClientBuilder, prelude::GatewayIntents as serenityGI, GatewayIntents,
};

#[tokio::main]
async fn main() {
    let _ = setup_database();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents: serenityGI = GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![poker(), poker_discount(), poker_top(), poker_search()], // Add the poker command
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = ClientBuilder::new(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
