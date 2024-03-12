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
        pub mod bosses;
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
        bosses::{agree_boss, test_test},
        bounties::give_bounty,
        points::{delete_points, substract_points, sum_points},
        poker::{search_user, top10, top10_tournament, verified},
        tournaments::{checking, create_tournament, get_tournaments},
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
                create_tournament(),
                sum_points(),
                substract_points(),
                delete_points(),
                give_bounty(),
                top10(),
                search_user(),
                top10_tournament(),
                verified(),
                checking(),
                agree_boss(),
                test_test(),
                get_tournaments(),
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
