use poise::{
    builtins,
    serenity_prelude::{
        client::ClientBuilder, prelude::GatewayIntents as serenityGI, GatewayIntents,
    },
    Framework, FrameworkOptions,
};

use crate::{data_structs::Data, db::config::setup_database};

use self::commands::{test::verify_bitmex, verify::verify_twitch};

use {
    commands::{
        bosses::{agree_boss, test_test},
        bounties::give_bounty,
        points::{delete_points, substract_points, sum_points},
        poker::{info_user, search_user, verified, verified_twitch},
        tournaments::{
            checking, create_tournament, get_tournaments, top10, top10_tournament, update_tables,
        },
    },
    handler::Handler,
};

pub mod commands;
pub mod handler;
pub mod utils;

pub async fn setup_discord() -> tokio::task::JoinHandle<()> {
    tokio::spawn(async {
        let _ = setup_database();

        let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
        let intents: serenityGI =
            GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

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
                    update_tables(),
                    info_user(),
                    verified_twitch(),
                    verify_twitch(),
                    verify_bitmex(),
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
    })
}
