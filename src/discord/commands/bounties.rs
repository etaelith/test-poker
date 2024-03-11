use crate::{
    data_structs::{Context, Error},
    db::commands::table_bounties::add_winner,
    discord::utils::parse_fecha,
};
use poise::{command, serenity_prelude::User, CreateReply};

#[command(slash_command, prefix_command)]
pub async fn give_bounty(
    ctx: Context<'_>,

    #[description = "Amount (1-100)"] points: u32,
    #[description = "User (mention or ID)"] user: Option<User>,
    #[description = "Insert Date (DD/MM/YYYY)"] fecha: String,
    #[description = "Won tournament? (true or false"] winner: Option<bool>,
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
        Ok(epoch_time) => match add_winner(
            winner.unwrap(),
            user_id,
            &target_user.name,
            points as i64,
            epoch_time,
        ) {
            Ok(_) => {
                ctx.send(CreateReply {
                    content: format!("Check parse_fecha: {epoch_time}").into(),
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
                    content: format!("Hubo un error en la funcion add_winner {err}").into(),
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
        },
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

    Ok(())
}
