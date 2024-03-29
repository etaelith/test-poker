use crate::{
    data_structs::{Context, Error},
    db::{
        commands::table_reward::{delete_reward, insert_reward, substract_reward},
        utils::tournament_exists,
    },
    discord::utils::{check_role, parse_fecha, send_message},
};

use poise::{command, serenity_prelude::User};

#[command(slash_command, prefix_command)]
pub async fn admin_sum_points(
    ctx: Context<'_>,
    #[description = "Points (1-100)"] points: u32,
    #[description = "User (mention or ID)"] user: Option<User>,
    #[description = "Insert Date tournament (DD/MM/YYYY)"] fecha: String,
) -> Result<(), Error> {
    let role_str = std::env::var("ROLE_ADMIN").expect("missing ID ROLE ADMIN");
    let checked = check_role(&ctx, role_str).await;
    match checked {
        Ok(true) => {
            if points < 1 || points > 100 {
                send_message(&ctx, format!("Please choose points between 1 and 100.")).await?;
                return Ok(());
            }
            let target_user = user.unwrap_or_else(|| ctx.author().clone());
            let user_id = i64::from(target_user.id);
            match parse_fecha(&fecha) {
                Ok(epoch_time) => match tournament_exists(epoch_time) {
                    Ok(true) => {
                        match insert_reward(user_id, &target_user.name, points as i64, epoch_time) {
                            Ok(_) => {
                                send_message(
                                    &ctx,
                                    format!("Reward delivered for {user_id}, \n points: {points}"),
                                )
                                .await?;
                            }
                            Err(err) => {
                                send_message(
                                    &ctx,
                                    format!("Hubo un error al insertar el reward: {err}"),
                                )
                                .await?;
                            }
                        }
                    }
                    Ok(false) => {
                        send_message(&ctx, format!("El torneo no existe.")).await?;
                    }
                    Err(err) => {
                        send_message(
                            &ctx,
                            format!("Hubo un error al verificar la existencia del torneo: {err}"),
                        )
                        .await?;
                    }
                },
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
            send_message(&ctx, format!("No tenes el role necesario")).await?;
        }
        Err(e) => {
            send_message(&ctx, format!("No tenes el role necesario {:?}", e)).await?;
        }
    }

    Ok(())
}
#[command(slash_command, prefix_command)]
pub async fn admin_substract_points(
    ctx: Context<'_>,

    #[description = "Points discount (1-100)"] points: u32,
    #[description = "User (mention or ID)"] user: Option<User>,
    #[description = "Insert Date tournament (DD/MM/YYYY)"] fecha: String,
) -> Result<(), Error> {
    let role_str = std::env::var("ROLE_ADMIN").expect("missing ID ROLE ADMIN");
    let checked = check_role(&ctx, role_str).await;

    match checked {
        Ok(true) => {
            if points < 1 || points > 100 {
                send_message(&ctx, format!("Please choose points between 1 and 100.")).await?;
            }

            let target_user = user.unwrap_or_else(|| ctx.author().clone());
            let user_id = i64::from(target_user.id);

            match parse_fecha(&fecha) {
                Ok(epoch_time) => match tournament_exists(epoch_time) {
                    Ok(true) => {
                        match substract_reward(
                            user_id,
                            &target_user.name,
                            points as i64,
                            epoch_time,
                        ) {
                            Ok(_) => {
                                send_message(
                                    &ctx,
                                    format!("Reward subtracted for {user_id}, \n points: {points}"),
                                )
                                .await?;
                            }
                            Err(err) => {
                                send_message(
                                    &ctx,
                                    format!("Hubo un error al extraer el reward: {err}"),
                                )
                                .await?;
                            }
                        }
                    }
                    Ok(false) => {
                        send_message(&ctx, format!("El torneo no existe.")).await?;
                    }
                    Err(err) => {
                        send_message(
                            &ctx,
                            format!("Hubo un error al verificar la existencia del torneo: {err}"),
                        )
                        .await?;
                    }
                },
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
            send_message(&ctx, format!("No tienes el rol necesario {:#?}", e)).await?;
        }
    }

    Ok(())
}
#[command(slash_command, prefix_command)]
pub async fn admin_delete_points(
    ctx: Context<'_>,
    #[description = "User (mention or ID)"] user: Option<User>,
    #[description = "Insert Date tournament (DD/MM/YYYY)"] fecha: String,
) -> Result<(), Error> {
    let role_str = std::env::var("ROLE_ADMIN").expect("missing ID ROLE ADMIN");
    let checked = check_role(&ctx, role_str).await;

    match checked {
        Ok(true) => {
            let target_user = user.unwrap_or_else(|| ctx.author().clone());
            let user_id = i64::from(target_user.id);

            match parse_fecha(&fecha) {
                Ok(epoch_time) => match tournament_exists(epoch_time) {
                    Ok(true) => match delete_reward(user_id, epoch_time) {
                        Ok(_) => {
                            send_message(&ctx, format!("Reward deleted for {user_id} on {fecha}",))
                                .await?;
                        }
                        Err(err) => {
                            send_message(&ctx, format!("Error deleting reward: {err}")).await?;
                        }
                    },
                    Ok(false) => {
                        send_message(&ctx, format!("The tournament does not exist.")).await?;
                    }
                    Err(err) => {
                        send_message(&ctx, format!("Error checking tournament existence: {err}"))
                            .await?;
                    }
                },
                Err(_) => {
                    send_message(
                        &ctx,
                        format!("Invalid date. Make sure to use the format DD/MM/YYYY."),
                    )
                    .await?;
                }
            }
        }
        Ok(false) => {
            send_message(&ctx, format!("You don't have the necessary role.")).await?;
        }
        Err(e) => {
            send_message(&ctx, format!("Role check failed: {:#?}", e)).await?;
        }
    }

    Ok(())
}
