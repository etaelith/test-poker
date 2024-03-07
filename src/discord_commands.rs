use crate::{
    data_structs::{Context, Error, ResponseStatus, TopTen},
    table_users::{create_or_sum, get_top, update_amount},
};
use poise::{command, say_reply, serenity_prelude::User, CreateReply};
use serenity::builder::GetMessages;
use serenity::model::id::ChannelId;
/// Displays your or another user's account creation date
#[command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.id, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

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

    // Get the target user (default to the author if not specified)
    let target_user = user.unwrap_or_else(|| ctx.author().clone());
    let thread_id: u64 = 1214415914196533308;
    // Respond with the chosen points and the target user
    let response = format!(
        "You selected {} points for {}!, id: {}",
        points, target_user.name, target_user.id
    );
    // ctx.say(response).await?;
    let channel_id = ChannelId::new(thread_id.into());
    let esto = i64::from(target_user.id);
    let _ = create_or_sum(&target_user.name, esto, points as i64);
    channel_id.say(&ctx.http(), response).await?;
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

    // Get the target user (default to the author if not specified)
    let target_user = user.unwrap_or_else(|| ctx.author().clone());
    let thread_id: u64 = 1214415914196533308;
    // Respond with the chosen points and the target user
    let response = format!(
        "You selected {} points for {}!, id: {}",
        points, target_user.name, target_user.id
    );
    let esto = i64::from(target_user.id);

    // ctx.say(response).await?;
    let channel_id = ChannelId::new(thread_id.into());
    let _ = update_amount(esto, points as i64);
    channel_id.say(&ctx.http(), response).await?;
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
                let _ = say_reply(ctx, &message).await;
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
pub async fn borrar(ctx: Context<'_>) -> Result<(), Error> {
    let builder = GetMessages::default().limit(100);
    let channel_id = ctx.channel_id();
    let msgs = channel_id.messages(ctx, builder).await?;

    for msg in msgs {
        match msg.delete(&ctx.http()).await {
            Ok(_) => {}
            Err(why) => {
                let reply_after = CreateReply::default()
                    .content(format!("{why:?}"))
                    .ephemeral(true);
                ctx.send(reply_after).await?;
            }
        }
    }
    ctx.send(CreateReply {
        content: Some(format!("Deleted messages in channel: {channel_id:?}")),
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
