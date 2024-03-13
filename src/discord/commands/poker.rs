use crate::{
    data_structs::{Context, Error, ResponseStatus},
    db::commands::table_users::{get_user_rank, verified_bitmex},
    discord::utils::{check_role, send_message},
};
use poise::{command, say_reply, serenity_prelude::User, CreateReply};

#[command(slash_command, prefix_command)]
pub async fn search_user(
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
pub async fn verified(
    ctx: Context<'_>,
    #[description = "User (mention or ID)"] user: Option<User>,
    #[description = "Verify? (true or false"] winner: Option<bool>,
) -> Result<(), Error> {
    let role_str = std::env::var("ROLE_ADMIN").expect("missing ID ROLE ADMIN");
    let checked = check_role(&ctx, role_str).await;
    match checked {
        Ok(true) => {
            let target_user = user.unwrap_or_else(|| ctx.author().clone());

            let user_id = i64::from(target_user.id);
            match verified_bitmex(user_id, winner.unwrap(), &target_user.name) {
                Ok(_) => {
                    send_message(
                        &ctx,
                        format!(
                            "State verify from {} changed to {}",
                            target_user.name,
                            winner.unwrap()
                        ),
                    )
                    .await?;
                }
                Err(err) => {
                    send_message(&ctx, format!("Hubo un error al cambiar verify: {err}")).await?;
                }
            }
        }
        Ok(false) => {
            send_message(&ctx, format!("No tenes el role necesario")).await?;
        }
        Err(e) => {
            send_message(&ctx, format!("No tenes el role necesario {:?}", e)).await?;
        }
    }

    Ok(())
}
