use crate::{
    data_structs::{Context, Error},
    db::{
        commands::{
            boss::update_tournament::insert_users_rewards, table_tournaments::add_tournament,
        },
        config::connect_database,
        utils::tournament_exists,
    },
    discord::utils::{check_role, parse_fecha, send_message},
};
use poise::command;

#[command(slash_command, prefix_command)]
pub async fn extraer_puestos_nombres(
    ctx: Context<'_>,
    #[description = "Insert Date here"] texto: String,
    #[description = "Insert Date (DD/MM/YYYY)"] fecha: String,
) -> Result<(), Error> {
    let role_str = std::env::var("ROLE_ADMIN").expect("missing ID ROLE ADMIN");
    let checked = check_role(&ctx, role_str).await;
    match checked {
        Ok(true) => {
            let campos: Vec<&str> = texto
                .split_whitespace()
                .filter(|&x| !x.ends_with('%'))
                .collect();
            match parse_fecha(&fecha) {
                Ok(epoch_time) => match tournament_exists(epoch_time) {
                    Ok(false) => {
                        let conn = connect_database().unwrap();
                        match add_tournament(&conn, epoch_time) {
                            Ok(_) => {
                                let _ = insert_users_rewards(campos, epoch_time);
                                send_message(
                                    &ctx,
                                    format!("Torneo y rewards asignados correctamente"),
                                )
                                .await?
                            }
                            Err(err_insert) => {
                                send_message(
                                    &ctx,
                                    format!("Error al agregar el torneo, error: \n {err_insert}"),
                                )
                                .await?
                            }
                        }
                    }
                    Ok(true) => {
                        let _ = insert_users_rewards(campos, epoch_time);
                        send_message(&ctx, format!("rewards asignados correctamente")).await?
                    }
                    Err(err_tournament) => {
                        send_message(
                            &ctx,
                            format!("Error funcion tournament_exists, error: \n {err_tournament}"),
                        )
                        .await?
                    }
                },

                Err(e) => {
                    send_message(
                        &ctx,
                        format!("Hubo un error en la funcion parse_fecha, error: \n {e}"),
                    )
                    .await?
                }
            }
        }

        Ok(false) => {
            send_message(&ctx, format!("No tienes el rol necesario")).await?;
        }
        Err(e) => {
            send_message(&ctx, format!("No tienes el rol necesario {e}")).await?;
        }
    }
    Ok(())
}
