use crate::{
    data_structs::{Context, Error, Tournaments},
    db::{
        commands::table_tournaments::{add_tournament, get_tournaments_date},
        config::connect_database,
        utils::tournament_exists,
    },
    discord::utils::{check_role, parse_fecha, send_message},
};

use poise::command;
use tokio::time::Instant;

#[command(slash_command, prefix_command)]
pub async fn create_tournament(
    ctx: Context<'_>,
    #[description = "Insert Date (DD/MM/YYYY)"] fecha: String,
) -> Result<(), Error> {
    let start_time = Instant::now();
    let role_str = std::env::var("ROLE_ADMIN").expect("missing ID ROLE ADMIN");
    let checked = check_role(&ctx, role_str).await;
    match checked {
        Ok(true) => {
            match parse_fecha(&fecha) {
                Ok(epoch_time) => {
                    // Verificar la existencia del torneo antes de intentar crearlo
                    match tournament_exists(epoch_time) {
                        Ok(false) => {
                            let conn = connect_database().unwrap();
                            match add_tournament(&conn, epoch_time) {
                                Ok(_) => {
                                    send_message(
                                        &ctx,
                                        format!(
                                            "Torneo creado con éxito para la fecha: {}",
                                            epoch_time
                                        ),
                                    )
                                    .await?;
                                    let end_time = Instant::now();
                                    let elapsed_time = end_time - start_time;
                                    println!(
                                        "create_tournament execution time: {:?}",
                                        elapsed_time
                                    );
                                }
                                Err(err) => {
                                    send_message(
                                        &ctx,
                                        format!("Hubo un error al crear el torneo: {err}"),
                                    )
                                    .await?;
                                }
                            }
                        }
                        Ok(true) => {
                            send_message(&ctx, format!("El torneo ya existe.")).await?;
                        }
                        Err(err) => {
                            send_message(
                                &ctx,
                                format!(
                                    "Hubo un error al verificar la existencia del torneo: {err}"
                                ),
                            )
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
            }
        }
        Ok(false) => {
            send_message(&ctx, format!("No tienes el rol necesario")).await?;
        }
        Err(e) => {
            send_message(&ctx, format!("Hubo un error en la funcion checked {e}")).await?;
        }
    }

    Ok(())
}
#[command(slash_command, prefix_command)]
pub async fn get_tournaments(ctx: Context<'_>) -> Result<(), Error> {
    match get_tournaments_date() {
        Ok(response_status) => {
            if response_status.success {
                if let Some(description) = response_status.success_description {
                    let tournaments: Vec<Tournaments> = serde_json::from_str(&description).unwrap();
                    let formatted_tournaments = tournaments
                        .iter()
                        .map(|tournament| {
                            format!("Tournament_date: {}", tournament.tournament_date)
                        })
                        .collect::<Vec<String>>()
                        .join("\n");
                    send_message(&ctx, formatted_tournaments).await?;
                } else {
                    send_message(&ctx, "description".to_owned()).await?;
                }
            } else {
                if let Some(error_message) = response_status.error_message {
                    send_message(&ctx, error_message).await?;
                } else {
                    send_message(&ctx, "content".to_owned()).await?;
                }
            }
        }
        Err(err) => {
            send_message(&ctx, format!("{err}")).await?;
        }
    }

    Ok(())
}

#[command(slash_command, prefix_command)]
pub async fn checking(ctx: Context<'_>) -> Result<(), Error> {
    let role_str = std::env::var("ROLE_ADMIN").expect("missing ID ROLE ADMIN");
    let checked = check_role(&ctx, role_str).await;
    match checked {
        Ok(true) => println!("Role checked: true -"),
        Ok(false) => println!("Role'nt checked: false -"),
        Err(e) => println!("Error checking role: {:?}", e),
    }
    Ok(())
}
