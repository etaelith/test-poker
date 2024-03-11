use crate::{
    data_structs::{Context, Error},
    db::{commands::table_tournaments::add_tournament, config::connect_database},
    discord::utils::{check_role, parse_fecha},
};

use poise::{command, CreateReply};

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
                        ctx.send(CreateReply {
                            content: format!("Torneo creado con éxito para la fecha: {epoch_time}")
                                .into(),
                            embeds: vec![],
                            attachments: vec![],
                            ephemeral: Some(true),
                            components: None,
                            allowed_mentions: None,
                            reply: false,
                            __non_exhaustive: (),
                        })
                        .await?;
                    }
                    Err(err) => {
                        ctx.send(CreateReply {
                            content: format!("Hubo un error al crear el torneo: {err}").into(),
                            embeds: vec![],
                            attachments: vec![],
                            ephemeral: Some(true),
                            components: None,
                            allowed_mentions: None,
                            reply: false,
                            __non_exhaustive: (),
                        })
                        .await?;
                    }
                }
            }
            Err(_) => {
                ctx.send(CreateReply {
                    content: format!("Fecha inválida. Asegúrate de usar el formato DD/MM/YYYY.")
                        .into(),
                    embeds: vec![],
                    attachments: vec![],
                    ephemeral: Some(true),
                    components: None,
                    allowed_mentions: None,
                    reply: false,
                    __non_exhaustive: (),
                })
                .await?;
            }
        },
        Ok(false) => {
            ctx.send(CreateReply {
                content: format!("No tenes el role necesario").into(),
                embeds: vec![],
                attachments: vec![],
                ephemeral: Some(true),
                components: None,
                allowed_mentions: None,
                reply: false,
                __non_exhaustive: (),
            })
            .await?;
        }
        Err(e) => {
            ctx.send(CreateReply {
                content: format!("Hubo un error en la funcion checked {e}").into(),
                embeds: vec![],
                attachments: vec![],
                ephemeral: Some(true),
                components: None,
                allowed_mentions: None,
                reply: false,
                __non_exhaustive: (),
            })
            .await?;
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
