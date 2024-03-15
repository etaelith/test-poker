use std::env;

use crate::data_structs::{Context, Error};
use poise::{
    command,
    serenity_prelude::{ButtonStyle, CreateActionRow, CreateButton},
    CreateReply,
};
#[command(slash_command, prefix_command)]
pub async fn verify_twitch(ctx: Context<'_>) -> Result<(), Error> {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let url_auth = format!("https://discord.com/oauth2/authorize?client_id={}&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A1500%2Fapi%2Fauth%2Fdiscord%2Fredirect&scope=identify+connections", client_id);
    // Crea un mensaje con un botón y un enlace
    let content = "¡Hola! Haz clic en el botón para validar tu usuario de twitch(Verifica que solamente tengas linkeada la cuenta que usas en el stream):";
    let reply = {
        let components = vec![CreateActionRow::Buttons(vec![CreateButton::new_link(
            url_auth,
        )
        .label("OPEN")
        .style(ButtonStyle::Success)])];
        CreateReply::default()
            .content(content)
            .components(components)
            .ephemeral(true)
    };
    ctx.send(reply).await?;
    Ok(())
}
