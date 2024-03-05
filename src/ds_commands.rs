use crate::data_structs::{Context, Error};
use poise::serenity_prelude as serenity;

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
        ctx.say("Please choose points between 1 and 10.").await?;
        return Ok(());
    }

    // Get the target user (default to the author if not specified)
    let target_user = user.unwrap_or_else(|| ctx.author().clone());

    // Respond with the chosen points and the target user
    let response = format!("You selected {} points for {}!", points, target_user.name);
    ctx.say(response).await?;

    // Execute additional actions (e.g., timeout, alert, etc.) based on the points if needed
    // For example, if points are 5 or more, give a timeout of 5 seconds

    Ok(())
}
