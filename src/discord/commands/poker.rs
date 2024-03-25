use crate::{
    data_structs::{Context, Error, ResponseStatus},
    db::commands::table_users::{get_user_info, get_user_rank},
};
use poise::{command, say_reply, serenity_prelude::User, CreateReply};

#[command(slash_command, prefix_command)]
pub async fn poker_search_user(
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
#[command(slash_command, prefix_command)]
pub async fn poker_info_user(
    ctx: Context<'_>,
    #[description = "User (mention or ID)"] user: Option<User>,
) -> Result<(), Error> {
    let target_user = user.unwrap_or_else(|| ctx.author().clone());
    let user_id = target_user.id.into();

    match get_user_info(&target_user.name, user_id) {
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
