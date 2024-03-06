mod data_structs;
mod db_config;
mod discord_handler;
mod ds_commands;
mod table_users;
use data_structs::Data;
use discord_handler::Handler;
use ds_commands::{age, borrar, poker};
use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents: ::serenity::prelude::GatewayIntents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), poker(), borrar()], // Add the poker command
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
