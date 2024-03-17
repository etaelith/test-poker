use poise::{
    builtins,
    serenity_prelude::{
        client::ClientBuilder, prelude::GatewayIntents as serenityGI, GatewayIntents,
    },
    Framework, FrameworkOptions,
};

use crate::{data_structs::Data, db::config::setup_database};

use self::commands::{
    test::poker_verify,
    verify::{poker_verify_bitmex, poker_verify_twitch},
};

use {
    commands::{
        bosses::admin_agree_boss,
        bounties::admin_give_bounty,
        points::{admin_delete_points, admin_substract_points, admin_sum_points},
        poker::{admin_verify_bitmex, admin_verify_twitch, poker_info_user, poker_search_user},
        tournaments::{
            admin_create_tournament, admin_update_tables, checking, poker_get_tournaments,
            poker_top10, poker_top10_tournament,
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
                    admin_create_tournament(),
                    admin_sum_points(),
                    admin_substract_points(),
                    admin_delete_points(),
                    admin_give_bounty(),
                    poker_top10(),
                    poker_search_user(),
                    poker_top10_tournament(),
                    admin_verify_bitmex(),
                    checking(),
                    admin_agree_boss(),
                    poker_get_tournaments(),
                    admin_update_tables(),
                    poker_info_user(),
                    admin_verify_twitch(),
                    poker_verify_twitch(),
                    poker_verify_bitmex(),
                    poker_verify(),
                ],
                ..Default::default()
            })
            .setup(|ctx, _ready, framework| {
                Box::pin(async move {
                    builtins::register_in_guild(
                        ctx,
                        &framework.options().commands,
                        1213686647225581598.into(),
                    )
                    .await?;
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
