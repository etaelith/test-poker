use crate::{
    data_structs::{Context, Error},
    table_users::{create_or_sum, update_amount},
};
use ::serenity::all::{ChannelId, GetMessages};
use poise::{serenity_prelude as serenity, CreateReply};

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.id, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Custom poker command
#[poise::command(slash_command, prefix_command)]
pub async fn poker(
    ctx: Context<'_>,

    #[description = "Points (1-10)"] points: u32,
    #[description = "User (mention or ID)"] user: Option<serenity::User>,
) -> Result<(), Error> {
    // Validate points
    if points < 1 || points > 10 {
        ctx.send(CreateReply {
            content: format!("Please choose points between 1 and 10.").into(),
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
        content: Some(format!("Points to: {}", ctx.author())),
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
#[poise::command(slash_command, prefix_command)]
pub async fn poker_discount(
    ctx: Context<'_>,

    #[description = "Points discount (1-10)"] points: u32,
    #[description = "User (mention or ID)"] user: Option<serenity::User>,
) -> Result<(), Error> {
    // Validate points
    if points < 1 || points > 10 {
        ctx.send(CreateReply {
            content: format!("Please choose points between 1 and 10.").into(),
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
        content: Some(format!("Points discounted to: {}", ctx.author())),
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

#[poise::command(slash_command, prefix_command)]
pub async fn borrar(ctx: Context<'_>) -> Result<(), Error> {
    let builder = GetMessages::default().limit(10);
    let algo = ctx.channel_id();
    let msgs = algo.messages(ctx, builder).await?;

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
        content: Some(format!("Deleted messages in channel: {algo:?}")),
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
