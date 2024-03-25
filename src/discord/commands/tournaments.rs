use crate::{
    data_structs::{Context, Error, ResponseStatus, TopTen, Tournaments},
    db::commands::table_tournaments::{get_top, get_top_tournament, get_tournaments_date},
    discord::utils::{check_role, parse_fecha, send_message},
};

use poise::{
    command, say_reply,
    serenity_prelude::{CreateEmbed, CreateMessage},
    CreateReply,
};

#[command(slash_command, prefix_command)]
pub async fn poker_get_tournaments(ctx: Context<'_>) -> Result<(), Error> {
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
pub async fn poker_top10(ctx: Context<'_>) -> Result<(), Error> {
    match get_top() {
        Ok(response) => {
            if let Some(success_description) = response.success_description {
                let success_description_str: &str = &success_description;
                let top_ten: Vec<TopTen> =
                    serde_json::from_str(success_description_str).unwrap_or_default();

                let mut message = String::from("Posiciones:\n");
                for user in &top_ten {
                    message.push_str(&format!(
                        "{}. {}, Puntos: {}\n",
                        user.position, user.name, user.points
                    ));
                }
                let reply = CreateReply {
                    content: Some(message.clone()),
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
pub async fn poker_top10_tournament(
    ctx: Context<'_>,
    #[description = "Insert Date tournament (DD/MM/YYYY)"] fecha: String,
) -> Result<(), Error> {
    match parse_fecha(&fecha) {
        Ok(epoch_time) => match get_top_tournament(epoch_time) {
            Ok(response) => {
                if let Some(success_description) = response.success_description {
                    let success_description_str: &str = &success_description;
                    let top_ten: Vec<TopTen> =
                        serde_json::from_str(success_description_str).unwrap_or_default();

                    let mut message = String::from("Posiciones:\n");
                    for user in &top_ten {
                        message.push_str(&format!(
                            "{}. {}, Puntos: {}\n",
                            user.position, user.name, user.points
                        ));
                    }

                    let reply = CreateReply {
                        content: Some(message.clone()),
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
        },
        Err(_) => {
            send_message(
                &ctx,
                format!("Fecha inválida. Asegúrate de usar el formato DD/MM/YYYY."),
            )
            .await?;
        }
    }

    Ok(())
}

#[command(slash_command, prefix_command)]
pub async fn admin_update_tables(ctx: Context<'_>) -> Result<(), Error> {
    let role_str = std::env::var("ROLE_ADMIN").expect("missing ID ROLE ADMIN");
    let checked = check_role(&ctx, role_str).await;

    match checked {
        Ok(true) => match get_top() {
            Ok(response) => {
                if response.success {
                    let top_users: Vec<TopTen> =
                        serde_json::from_str(&response.success_description.unwrap()).unwrap();

                    let mut fields_vec = Vec::new();
                    for user in top_users {
                        fields_vec.push((
                            format!("Posición {}", user.position),
                            format!("{}: {} puntos", user.name, user.points),
                            false,
                        ));
                    }

                    let embed = CreateEmbed::new()
                        .title("Top 10 Usuarios")
                        .fields(fields_vec);

                    let builder = CreateMessage::new().embed(embed);

                    let msg = ctx.channel_id().send_message(&ctx.http(), builder).await;
                    if let Err(why) = msg {
                        let _ = send_message(&ctx, why.to_string());
                    }
                } else {
                    let builder =
                        CreateMessage::new().content("Error al obtener el top 10 de usuarios");
                    let msg = ctx.channel_id().send_message(&ctx.http(), builder).await;
                    if let Err(why) = msg {
                        let _ = send_message(&ctx, why.to_string());
                    }
                }
            }
            Err(err) => {
                let _ = send_message(&ctx, err.to_string());
            }
        },
        Ok(false) => {
            let _ = send_message(&ctx, "Role'nt checked: false -".to_string());
        }
        Err(e) => {
            let _ = send_message(&ctx, e.to_string());
        }
    }

    Ok(())
}

#[command(slash_command, prefix_command)]
pub async fn checking(ctx: Context<'_>) -> Result<(), Error> {
    let role_str = std::env::var("ROLE_ADMIN").expect("missing ID ROLE ADMIN");
    let checked = check_role(&ctx, role_str).await;
    match checked {
        Ok(true) => {
            let _ = send_message(&ctx, "Role checked: true -".to_string());
        }
        Ok(false) => {
            let _ = send_message(&ctx, "Role checked: false".to_string());
        }
        Err(e) => {
            let _ = send_message(&ctx, e.to_string());
        }
    }
    Ok(())
}
