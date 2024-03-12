use crate::{
    data_structs::{Context, Error},
    db::{config::connect_database, utils::insert_idiot},
    discord::utils::{check_role, send_message},
};

use poise::{command, serenity_prelude::User};
#[command(slash_command, prefix_command)]
pub async fn agree_boss(
    ctx: Context<'_>,
    #[description = "User (mention or ID)"] user: Option<User>,
) -> Result<(), Error> {
    let role_str = std::env::var("ROLE_ADMIN").expect("missing ID ROLE ADMIN");
    let checked = check_role(&ctx, role_str).await;

    match checked {
        Ok(true) => {
            let target_user = user.unwrap_or_else(|| ctx.author().clone());
            let user_id = target_user.id.into();
            let conn = connect_database().unwrap();
            match insert_idiot(&conn, user_id, &target_user.name) {
                Ok(response_status) => {
                    // Manejar el ResponseStatus según sea necesario
                    if response_status.success {
                        // Hacer algo en caso de éxito
                        send_message(&ctx, format!("Idiotita spawn correctamente")).await?;
                    } else {
                        // Hacer algo en caso de otro estado
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
                    // Manejar el error al insertar el idiotita
                    send_message(&ctx, format!("Error al insertar el idiotita: {:?}", err)).await?;
                }
            }
        }
        Ok(false) => {
            // Manejar el caso en que el rol no esté presente
            send_message(&ctx, format!("No tienes el rol necesario")).await?;
        }
        Err(e) => {
            // Manejar otros errores
            send_message(&ctx, format!("Error: {:?}", e)).await?;
        }
    }

    Ok(())
}
