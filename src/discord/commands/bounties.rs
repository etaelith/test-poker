use crate::{
    data_structs::{Context, Error},
    db::commands::table_bounties::add_winner,
    discord::utils::{check_role, parse_fecha, send_message},
};
use poise::{command, serenity_prelude::User};

#[command(slash_command, prefix_command)]
pub async fn give_bounty(
    ctx: Context<'_>,

    #[description = "Amount (1-100)"] points: u32,
    #[description = "User (mention or ID)"] user: Option<User>,
    #[description = "Insert Date (DD/MM/YYYY)"] fecha: String,
    #[description = "Won tournament? (true or false"] winner: Option<bool>,
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
                Ok(epoch_time) => match add_winner(
                    winner.unwrap(),
                    user_id,
                    &target_user.name,
                    points as i64,
                    epoch_time,
                ) {
                    Ok(_) => {
                        send_message(&ctx, format!("Bounty gived: {points}")).await?;
                    }
                    Err(err) => {
                        send_message(
                            &ctx,
                            format!("Hubo un error en la funcion add_winner {err}"),
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
            send_message(&ctx, format!("No tenes el role necesario {e}")).await?;
        }
    }

    Ok(())
}
