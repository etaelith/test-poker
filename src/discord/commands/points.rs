use crate::{
    data_structs::{Context, Error},
    db::commands::table_reward::{insert_reward, substract_reward},
    discord::utils::parse_fecha,
};

use poise::{command, serenity_prelude::User, CreateReply};

#[command(slash_command, prefix_command)]
pub async fn sum_points(
    ctx: Context<'_>,

    #[description = "Points (1-100)"] points: u32,
    #[description = "User (mention or ID)"] user: Option<User>,
    #[description = "Insert Date (DD/MM/YYYY)"] fecha: String,
) -> Result<(), Error> {
    if points < 1 || points > 100 {
        ctx.send(CreateReply {
            content: format!("Please choose points between 1 and 100.").into(),
            embeds: vec![],
            attachments: vec![],
            ephemeral: Some(true),
            components: None,
            allowed_mentions: None,
            reply: false,
            __non_exhaustive: (),
        })
        .await?;
        return Ok(());
    }

    let target_user = user.unwrap_or_else(|| ctx.author().clone());

    let user_id = i64::from(target_user.id);

    match parse_fecha(&fecha) {
        Ok(epoch_time) => {
            match insert_reward(user_id, &target_user.name, points as i64, epoch_time) {
                Ok(_) => {
                    ctx.send(CreateReply {
                        content: format!("Reward delivered for {user_id}, \n points: {points}")
                            .into(),
                        embeds: vec![],
                        attachments: vec![],
                        ephemeral: Some(true),
                        components: None,
                        allowed_mentions: None,
                        reply: false,
                        __non_exhaustive: (),
                    })
                    .await?;
                }
                Err(err) => {
                    ctx.send(CreateReply {
                        content: format!("Hubo un error al insertar el reward: {err}").into(),
                        embeds: vec![],
                        attachments: vec![],
                        ephemeral: Some(true),
                        components: None,
                        allowed_mentions: None,
                        reply: false,
                        __non_exhaustive: (),
                    })
                    .await?;
                }
            }
        }
        Err(_) => {
            ctx.send(CreateReply {
                content: format!("Fecha inválida. Asegúrate de usar el formato DD/MM/YYYY.").into(),
                embeds: vec![],
                attachments: vec![],
                ephemeral: Some(true),
                components: None,
                allowed_mentions: None,
                reply: false,
                __non_exhaustive: (),
            })
            .await?;
        }
    }

    ctx.send(CreateReply {
        content: Some(format!("Points to: {}", target_user.name)),
        embeds: vec![],
        attachments: vec![],
        ephemeral: Some(true),
        components: None,
        allowed_mentions: None,
        reply: false,
        __non_exhaustive: (),
    })
    .await?;
    Ok(())
}
#[command(slash_command, prefix_command)]
pub async fn poker_discount(
    ctx: Context<'_>,

    #[description = "Points discount (1-100)"] points: u32,
    #[description = "User (mention or ID)"] user: Option<User>,
    #[description = "Insert Date (DD/MM/YYYY)"] fecha: String,
) -> Result<(), Error> {
    if points < 1 || points > 100 {
        ctx.send(CreateReply {
            content: format!("Please choose points between 1 and 100.").into(),
            embeds: vec![],
            attachments: vec![],
            ephemeral: Some(true),
            components: None,
            allowed_mentions: None,
            reply: false,
            __non_exhaustive: (),
        })
        .await?;
        return Ok(());
    }

    let target_user = user.unwrap_or_else(|| ctx.author().clone());

    let user_id = i64::from(target_user.id);

    match parse_fecha(&fecha) {
        Ok(epoch_time) => {
            match substract_reward(user_id, &target_user.name, points as i64, epoch_time) {
                Ok(_) => {
                    ctx.send(CreateReply {
                        content: format!("Reward substract for {user_id}, \n points: {points}")
                            .into(),
                        embeds: vec![],
                        attachments: vec![],
                        ephemeral: Some(true),
                        components: None,
                        allowed_mentions: None,
                        reply: false,
                        __non_exhaustive: (),
                    })
                    .await?;
                }
                Err(err) => {
                    ctx.send(CreateReply {
                        content: format!("Hubo un error al extraer el reward: {err}").into(),
                        embeds: vec![],
                        attachments: vec![],
                        ephemeral: Some(true),
                        components: None,
                        allowed_mentions: None,
                        reply: false,
                        __non_exhaustive: (),
                    })
                    .await?;
                }
            }
        }
        Err(_) => {
            ctx.send(CreateReply {
                content: format!("Fecha inválida. Asegúrate de usar el formato DD/MM/YYYY.").into(),
                embeds: vec![],
                attachments: vec![],
                ephemeral: Some(true),
                components: None,
                allowed_mentions: None,
                reply: false,
                __non_exhaustive: (),
            })
            .await?;
        }
    }

    ctx.send(CreateReply {
        content: Some(format!("Points subs to: {}", target_user.name)),
        embeds: vec![],
        attachments: vec![],
        ephemeral: Some(true),
        components: None,
        allowed_mentions: None,
        reply: false,
        __non_exhaustive: (),
    })
    .await?;
    Ok(())
}
