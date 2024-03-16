use crate::data_structs::{Context, Error};
use hmac::{Hmac, Mac, NewMac};
use poise::{
    command,
    serenity_prelude::{ButtonStyle, CreateActionRow, CreateButton},
    CreateReply,
};
use reqwest::header::HeaderMap;
use serde_json::Value;
use sha2::Sha256;
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
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

pub async fn call_verify_bitmex(
    api_key: String,
    api_secret: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let guild_id = env::var("GUILD_ID").expect("GUILD_ID not set");

    let url = "https://www.bitmex.com/api/v1/user";
    let path = "/api/v1/user";
    let expires = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 60;
    let message = format!("GET{}{}", path, expires);
    let mut mac = Hmac::<Sha256>::new_varkey(api_secret.as_bytes()).unwrap();
    mac.update(message.as_bytes());
    let signature = mac.finalize().into_bytes();
    let signature_hex: String = signature.iter().map(|b| format!("{:02x}", b)).collect();
    let mut headers = HeaderMap::new();
    headers.insert("api-expires", expires.to_string().parse().unwrap());
    headers.insert("api-key", api_key.parse().unwrap());
    headers.insert("api-signature", signature_hex.parse().unwrap());

    let client = reqwest::Client::new();
    let response = client.get(url).headers(headers.into()).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;
        let user_id = json["id"].as_i64().unwrap();
        let target_account_id: i64 = guild_id.parse().unwrap();
        if let Some(accounts) = json["accounts"].as_array() {
            for account in accounts {
                if let Some(id) = account["id"].as_i64() {
                    if id == target_account_id {
                        let account_name = account["name"].as_str().unwrap_or("N/A");
                        println!("ID de la cuenta buscada: {}", id);
                        println!("Nombre de la cuenta: {}", account_name);
                        break;
                    }
                }
            }
        }
        println!("user_id: {}", user_id);
    } else {
        println!("Error: {}", response.status());
    }

    Ok(())
}
