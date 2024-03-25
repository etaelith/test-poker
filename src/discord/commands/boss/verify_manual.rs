use crate::{
    data_structs::{Context, Error},
    db::{commands::table_users::verified_bitmex, utils::insert_twitch_tag},
    discord::utils::{check_role, send_message},
};
use poise::{command, serenity_prelude::User};

#[command(slash_command, prefix_command)]
pub async fn admin_verify_twitch(
    ctx: Context<'_>,
    #[description = "User (mention or ID)"] user_id: Option<User>,
    #[description = "User (mention or ID)"] user_twitch: Option<String>,
) -> Result<(), Error> {
    if let Some(user_twitch) = user_twitch {
        let role_str = std::env::var("ROLE_ADMIN").expect("missing ID ROLE ADMIN");
        match check_role(&ctx, role_str).await {
            Ok(true) => {
                let target_user = user_id.unwrap_or_else(|| ctx.author().clone());
                let id_user = i64::from(target_user.id);
                insert_twitch_tag(id_user, &user_twitch)?;
                send_message(
                    &ctx,
                    format!("State verify from {} changed to", target_user.name,),
                )
                .await?;
            }
            Ok(false) => send_message(&ctx, format!("No tenes el role necesario")).await?,
            Err(e) => send_message(&ctx, format!("No tenes el role necesario {:?}", e)).await?,
        }
    } else {
        send_message(
            &ctx,
            "El argumento user_twitch no puede ser None".to_string(),
        )
        .await?;
    }

    Ok(())
}

#[command(slash_command, prefix_command)]
pub async fn admin_verify_bitmex(
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
