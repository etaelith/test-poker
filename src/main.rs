mod data_structs;
mod db {
    pub mod config;
    pub mod utils;
    pub mod commands {
        pub mod table_bounties;
        pub mod table_reward;
        pub mod table_tournaments;
        pub mod table_users;
    }
}
mod discord {
    pub mod handler;
    pub mod utils;
    pub mod commands {
        pub mod bounties;
        pub mod points;
        pub mod poker;
        pub mod tournaments;
    }
}
use data_structs::Data;
use db::config::setup_database;
use discord::{
    commands::{
        bounties::give_bounty,
        points::{poker_discount, sum_points},
        poker::{poker_search, poker_top, verifed},
        tournaments::{create_tournament, test_time},
    },
    handler::Handler,
};
use poise::{
    builtins,
    serenity_prelude::{
        client::ClientBuilder, prelude::GatewayIntents as serenityGI, GatewayIntents,
    },
    Framework, FrameworkOptions,
};

#[tokio::main]
async fn main() {
    let _ = setup_database();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents: serenityGI = GatewayIntents::non_privileged();

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![
                sum_points(),
                poker_discount(),
                poker_top(),
                poker_search(),
                create_tournament(),
                test_time(),
                give_bounty(),
                verifed(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                builtins::register_globally(ctx, &framework.options().commands).await?;
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
