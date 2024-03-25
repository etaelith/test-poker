use crate::{
    data_structs::{Context, Error},
    db::{config::connect_database, utils::insert_idiot},
    discord::utils::{check_id, send_message},
};
use poise::{command, serenity_prelude::User};
#[command(slash_command, prefix_command)]
pub async fn admin_agree_boss(
    ctx: Context<'_>,
    #[description = "User (mention or ID)"] user: Option<User>,
    #[description = "Guild ID"] guild_id: i64,
) -> Result<(), Error> {
    let checked = check_id(&ctx).await?;
    if checked {
        let target_user = user.unwrap_or_else(|| ctx.author().clone());
        let user_id = target_user.id.into();
        let conn = connect_database().unwrap();
        match insert_idiot(&conn, user_id, &target_user.name, guild_id) {
            Ok(response_status) => {
                if response_status.success {
                    send_message(&ctx, format!("Idiotita spawn correctamente")).await?;
                } else {
                    send_message(
                        &ctx,
                        format!(
                            "Error al spawnear idiotita: {:?}",
                            response_status.error_message
                        ),
                    )
                    .await?;
                }
            }
            Err(err) => {
                send_message(&ctx, format!("Error al insertar el idiotita: {:?}", err)).await?;
            }
        }
    } else {
        send_message(&ctx, format!("No sos el owner del discord")).await?;
    }
    Ok(())
}
