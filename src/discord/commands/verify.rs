use crate::{
    data_structs::{Context, Error, ResponseStatus, VerifyBitmex},
    db::commands::table_users::verified_bitmex,
    discord::utils::send_message,
};
use hmac::{Hmac, Mac, NewMac};
use poise::{
    command,
    serenity_prelude::{ButtonStyle, ComponentInteractionCollector, CreateActionRow, CreateButton},
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
pub async fn poker_verify_twitch(ctx: Context<'_>) -> Result<(), Error> {
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
#[poise::command(prefix_command, slash_command, dm_only = true)]
pub async fn poker_verify_bitmex(ctx: Context<'_>) -> Result<(), Error> {
    let reply = {
        let components = vec![CreateActionRow::Buttons(vec![CreateButton::new(
            "verify_bitmex",
        )
        .label("Verify")
        .style(ButtonStyle::Success)])];

        poise::CreateReply::default()
            .ephemeral(true)
            .content("Click the button below to verify bitmex")
            .components(components)
    };

    ctx.send(reply).await?;

    while let Some(mci) = ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == "verify_bitmex")
        .await
    {
        let data =
            poise::execute_modal_on_component_interaction::<VerifyBitmex>(ctx, mci, None, None)
                .await?;
        if let Some(data) = &data {
            let api_key = &data.api_key;
            let api_secret = &data.api_secret;
            let id_user = ctx.author().id;
            let tag_user = &ctx.author().name;
            match call_verify_bitmex(
                api_key.to_string(),
                api_secret.to_string(),
                id_user.into(),
                tag_user,
            )
            .await
            {
                Ok(datos) => {
                    let _ = send_message(&ctx, datos.success_description.unwrap());
                }
                Err(err) => {
                    println!("{}", err)
                }
            }

            let _ = send_message(&ctx, "Bitmex verificado".to_string());
        } else {
            println!("No se recibieron datos de la interacción del componente");
        }
    }

    Ok(())
}
async fn call_verify_bitmex(
    api_key: String,
    api_secret: String,
    id_user: i64,
    tag_user: &str,
) -> Result<ResponseStatus, Box<dyn std::error::Error>> {
    let guild_id = env::var("GUILD_ID_BITMEX").expect("GUILD_ID_BITMEX not set");

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
        let target_account_id: i64 = guild_id.parse().unwrap();
        if let Some(accounts) = json["accounts"].as_array() {
            for account in accounts {
                if let Some(id) = account["id"].as_i64() {
                    if id == target_account_id {
                        let account_name = account["name"].as_str().unwrap_or("N/A");
                        println!("ID de la cuenta buscada: {}", id);
                        println!("Nombre de la cuenta: {}", account_name);
                        match verified_bitmex(id_user, true, tag_user) {
                            Ok(datos) => {
                                return Ok(datos);
                            }
                            Err(_) => {
                                eprintln!("Error")
                            }
                        };
                    }
                }
            }
        }
    } else {
        eprintln!("Error: {}", response.status());
    }

    Ok(ResponseStatus {
        success: true,
        success_description: Some(format!("Bitmex verified2!")),
        error_message: None,
    })
}
