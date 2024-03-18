use actix_web::{get, web, HttpResponse, Responder};
use poise::serenity_prelude::User;
use std::{collections::HashMap, env};

use crate::{data_structs::Connection, db::utils::insert_twitch};

#[get("/api/auth/discord/redirect")]
async fn redirect(req: web::Query<HashMap<String, String>>) -> impl Responder {
    if let Some(code) = req.get("code") {
        let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
        let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
        let port = env::var("PORT").expect("PORT not set");
        let redirect_uri = format!("http://localhost:{}/api/auth/discord/redirect", port);
        let mut params = std::collections::HashMap::new();
        params.insert("client_id".to_string(), client_id);
        params.insert("client_secret".to_string(), client_secret);
        params.insert("grant_type".to_string(), "authorization_code".to_string());
        params.insert("code".to_string(), code.clone());
        params.insert("redirect_uri".to_string(), redirect_uri.to_string());
        print!("Hola");
        match get_oauth_token(&params).await {
            Ok(output) => {
                print!("Hola");
                if let Some(access) = output.get("access_token").and_then(|v| v.as_str()) {
                    let access = access.to_string();
                    print!("Hola");
                    match get_user_connections(&access).await {
                        Ok(connections) => {
                            if let Some(twitch_connection) = connections
                                .iter()
                                .find(|conn| conn.connection_type == "twitch")
                            {
                                match get_user_data(&access).await {
                                    Ok(user) => {
                                        match insert_twitch(
                                            user.id.into(),
                                            user.name.to_string(),
                                            twitch_connection.name.to_string(),
                                        ) {
                                            Ok(_) => {
                                                print!("success redirect")
                                            }
                                            Err(e) => {
                                                eprintln!("Error handling redirect: {}", e);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Error getting user data: {}", e);
                                    }
                                }
                            } else {
                                println!("No Twitch connection found");
                            }
                        }
                        Err(e) => {
                            eprintln!("Error getting user connections: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error getting OAuth token: {}", e);
            }
        }
    }

    HttpResponse::Ok()
}
async fn get_oauth_token(
    params: &HashMap<String, String>,
) -> Result<serde_json::Value, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .post("https://discord.com/api/oauth2/token")
        .form(params)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await
}

async fn get_user_connections(access_token: &str) -> Result<Vec<Connection>, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .get("https://discord.com/api/v10/users/@me/connections")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?
        .json::<Vec<Connection>>()
        .await
}

async fn get_user_data(access_token: &str) -> Result<User, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .get("https://discord.com/api/v10/users/@me")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?
        .json::<User>()
        .await
}
