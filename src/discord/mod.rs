use poise::{
    builtins,
    serenity_prelude::{
        client::ClientBuilder, prelude::GatewayIntents as serenityGI, GatewayIntents,
    },
    Framework, FrameworkOptions,
};

use crate::{data_structs::Data, db::config::setup_database};

use commands::{
    boss::{
        bounties::admin_give_bounty,
        insert_guild_bm::admin_agree_boss,
        insert_results::extraer_puestos_nombres,
        insert_tournament::admin_create_tournament,
        points::{admin_delete_points, admin_substract_points, admin_sum_points},
        verify_manual::{admin_verify_bitmex, admin_verify_twitch},
    },
    guide::poker_verify,
    poker::{poker_info_user, poker_search_user},
    tournaments::{
        admin_update_tables, checking, poker_get_tournaments, poker_top10, poker_top10_tournament,
    },
    verify::{poker_verify_bitmex, poker_verify_twitch},
};

use handler::Handler;

mod commands;
mod handler;
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
                    admin_agree_boss(),
                    admin_create_tournament(),
                    admin_sum_points(),
                    admin_substract_points(),
                    admin_delete_points(),
                    admin_verify_bitmex(),
                    admin_verify_twitch(),
                    admin_give_bounty(),
                    poker_top10(),
                    poker_search_user(),
                    poker_top10_tournament(),
                    checking(),
                    poker_get_tournaments(),
                    admin_update_tables(),
                    poker_info_user(),
                    poker_verify_twitch(),
                    poker_verify_bitmex(),
                    poker_verify(),
                    extraer_puestos_nombres(),
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
