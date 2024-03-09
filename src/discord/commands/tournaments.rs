use crate::{
    data_structs::{Context, Error},
    db::{config::connect_database, table_tournaments::add_tournament},
};
use chrono::prelude::*;
use poise::{command, CreateReply};

#[command(slash_command, prefix_command)]
pub async fn create_tournament(
    ctx: Context<'_>,
    #[description = "Insert Date (DD/MM/YYYY)"] fecha: String,
) -> Result<(), Error> {
    match NaiveDate::parse_from_str(&fecha, "%d/%m/%Y") {
        Ok(parsed_date) => {
            let conn = connect_database().unwrap();
            match add_tournament(&conn, parsed_date) {
                Ok(_) => {
                    ctx.send(CreateReply {
                        content: format!("Torneo creado con éxito para la fecha: {parsed_date}")
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
