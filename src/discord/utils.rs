use crate::data_structs::{Context, Error};
use chrono::{DateTime, Duration, NaiveDate, Utc};
use poise::CreateReply;
// check role ENV
pub async fn check_role(ctx: &Context<'_>, role_str: String) -> Result<bool, Error> {
    let role: u64 = role_str.parse().expect("Role should be a valid u64");
    let guild_id = ctx.guild_id().unwrap();
    let has_role = ctx.author().has_role(ctx.http(), guild_id, role).await?;
    Ok(has_role)
}
// check ID OWNER GUILD
pub async fn check_id(ctx: &Context<'_>) -> Result<bool, Error> {
    let id_admin_result = ctx.http().get_current_application_info().await;
    let is_owner = match id_admin_result {
        Ok(info) => ctx.author().id == info.owner.unwrap().id,
        Err(_) => false,
    };

    Ok(is_owner)
}
// SEND ephemeral answer
pub async fn send_message(ctx: &Context<'_>, content: String) -> Result<(), Error> {
    ctx.send(CreateReply {
        content: content.into(),
        embeds: vec![],
        attachments: vec![],
        ephemeral: Some(true),
        components: None,
        allowed_mentions: None,
        reply: false,
        __non_exhaustive: (),
    })
    .await?;
    Ok(())
}
// parse DD/MM/YYYY into UNIX
pub fn parse_fecha(fecha: &str) -> Result<i64, &'static str> {
    match NaiveDate::parse_from_str(fecha, "%d/%m/%Y") {
        Ok(parsed_date) => {
            let zero_seconds = Duration::zero();
            let naive_datetime = parsed_date + zero_seconds;
            let epoch_seconds = naive_datetime.and_hms_opt(0, 0, 0);
            match epoch_seconds {
                Some(time) => Ok(time.and_utc().timestamp()),
                None => Err("Fecha inválida. Asegúrate de usar el formato DD/MM/YYYY."),
            }
        }
        Err(_) => Err("Fecha inválida. Asegúrate de usar el formato DD/MM/YYYY."),
    }
}
// parse UNIX into DD/MM/YYYY
pub fn format_unix_timestamp(fecha: i64) -> Result<String, &'static str> {
    match DateTime::<Utc>::from_timestamp(fecha, 0) {
        Some(datetime) => Ok(datetime.format("%d/%m/%Y").to_string()),
        None => Err("Timestamp inválido."),
    }
}
