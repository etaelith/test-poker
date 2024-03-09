use crate::{
    data_structs::{Context, Error, ResponseStatus, TopTen},
    table_users::{create_or_sum, get_top, get_user_rank, update_amount},
};
use poise::{command, say_reply, serenity_prelude::User, CreateReply};

/// Custom poker command
#[command(slash_command, prefix_command)]
pub async fn poker(
    ctx: Context<'_>,

    #[description = "Points (1-100)"] points: u32,
    #[description = "User (mention or ID)"] user: Option<User>,
) -> Result<(), Error> {
    // Validate points
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
    let _ = create_or_sum(&target_user.name, user_id, points as i64);
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
) -> Result<(), Error> {
    // Validate points
    if points < 1 || points > 100 {
        ctx.send(CreateReply {
            content: format!("Please choose how much points discount.").into(),
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

    let _ = update_amount(user_id, points as i64);

    ctx.send(CreateReply {
        content: Some(format!("Points discounted to: {}", target_user.name)),
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
pub async fn poker_top(ctx: Context<'_>) -> Result<(), Error> {
    match get_top() {
        Ok(response) => {
            if let Some(success_description) = response.success_description {
                let success_description_str: &str = &success_description;
                let top_ten: Vec<TopTen> =
                    serde_json::from_str(success_description_str).unwrap_or_default();

                let mut message = String::from("Posiciones:\n");
                for user in &top_ten {
                    message.push_str(&format!(
                        "{}. {}, Puntos: {}\n",
                        user.position, user.name, user.points
                    ));
                }
                //let _ = say_reply(ctx, &message).await;
                let reply = CreateReply {
                    content: Some(message.clone()),
                    embeds: vec![],
                    attachments: vec![],
                    ephemeral: Some(true),
                    components: None,
                    allowed_mentions: None,
                    reply: false,
                    __non_exhaustive: (),
                };
                let _ = ctx.send(reply).await;
            }
        }
        Err(err) => {
            let error_response = ResponseStatus {
                success: false,
                success_description: None,
                error_message: Some(err.to_string()),
            };

            let _ = say_reply(ctx, &serde_json::to_string(&error_response).unwrap()).await;
        }
    }

    Ok(())
}

#[command(slash_command, prefix_command)]
pub async fn poker_search(
    ctx: Context<'_>,
    #[description = "User (mention or ID)"] user: Option<User>,
) -> Result<(), Error> {
    let target_user = user.unwrap_or_else(|| ctx.author().clone());

    let user_id = target_user.id.into();
    let _ = get_user_rank(&target_user.name, user_id);

    match get_user_rank(&target_user.name, user_id) {
        Ok(response) => {
            if let Some(success_description) = response.success_description {
                let reply = CreateReply {
                    content: Some(success_description.clone()),
                    embeds: vec![],
                    attachments: vec![],
                    ephemeral: Some(true),
                    components: None,
                    allowed_mentions: None,
                    reply: false,
                    __non_exhaustive: (),
                };
                let _ = ctx.send(reply).await;
            }
        }
        Err(err) => {
            let error_response = ResponseStatus {
                success: false,
                success_description: None,
                error_message: Some(err.to_string()),
            };
            let _ = say_reply(ctx, &serde_json::to_string(&error_response).unwrap()).await;
        }
    }
    Ok(())
}
