use crate::{
    data_structs::{Context, Error},
    db::{commands::table_tournaments::add_tournament, config::connect_database},
    discord::utils::{check_role, parse_fecha, send_message},
};

use poise::command;

#[command(slash_command, prefix_command)]
pub async fn create_tournament(
    ctx: Context<'_>,
    #[description = "Insert Date (DD/MM/YYYY)"] fecha: String,
) -> Result<(), Error> {
    let role_str = std::env::var("ROLE_ADMIN").expect("missing ID ROLE ADMIN");
    let checked = check_role(&ctx, role_str).await;
    match checked {
        Ok(true) => match parse_fecha(&fecha) {
            Ok(epoch_time) => {
                let conn = connect_database().unwrap();
                match add_tournament(&conn, epoch_time) {
                    Ok(_) => {
                        send_message(
                            &ctx,
                            format!("Torneo creado con éxito para la fecha: {}", epoch_time),
                        )
                        .await?;
                    }
                    Err(err) => {
                        send_message(&ctx, format!("Hubo un error al crear el torneo: {err}"))
                            .await?;
                    }
                }
            }
            Err(_) => {
                send_message(
                    &ctx,
                    format!("Fecha inválida. Asegúrate de usar el formato DD/MM/YYYY."),
                )
                .await?;
            }
        },
        Ok(false) => {
            send_message(&ctx, format!("No tenes el role necesario")).await?;
        }
        Err(e) => {
            send_message(&ctx, format!("Hubo un error en la funcion checked {e}")).await?;
        }
    }

    Ok(())
}

#[command(slash_command, prefix_command)]
pub async fn checking(
    ctx: Context<'_>,
    #[description = "Insert Date (DD/MM/YYYY)"] fecha: String,
) -> Result<(), Error> {
    let role_str = std::env::var("ROLE_ADMIN").expect("missing ID ROLE ADMIN");
    let checked = check_role(&ctx, role_str).await;
    match checked {
        Ok(true) => println!("Role checked: true - {fecha}"),
        Ok(false) => println!("Role'nt checked: false - {fecha}"),
        Err(e) => println!("Error checking role: {:?}", e),
    }
    Ok(())
}
