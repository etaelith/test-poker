use crate::{
    data_structs::{Context, Error},
    db::{commands::table_tournaments::add_tournament, config::connect_database},
    discord::utils::parse_fecha,
};

use poise::{command, CreateReply};

#[command(slash_command, prefix_command)]
pub async fn create_tournament(
    ctx: Context<'_>,
    #[description = "Insert Date (DD/MM/YYYY)"] fecha: String,
) -> Result<(), Error> {
    match parse_fecha(&fecha) {
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
                content: format!("Fecha inválida. Asegúrate de usar el formato DD/MM/YYYY.").into(),
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
pub async fn test_time(
    ctx: Context<'_>,
    #[description = "Insert Date (DD/MM/YYYY)"] fecha: String,
) -> Result<(), Error> {
    match parse_fecha(&fecha) {
        Ok(epoch_time) => {
            println!("Epoch time: {}", epoch_time)
        }
        Err(_) => {
            ctx.send(CreateReply {
                content: format!("Fecha inválida. Asegúrate de usar el formato DD/MM/YYYY.").into(),
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
