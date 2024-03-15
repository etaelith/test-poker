use actix_web::{get, web, HttpResponse, Responder};
use poise::serenity_prelude::User;
use std::{collections::HashMap, env};

use crate::{
    data_structs::Connection,
    db::{
        config::connect_database,
        utils::{insert_twitch_tag, insert_user, user_exists},
    },
};

#[get("/api/auth/discord/redirect")]
async fn redirect(req: web::Query<HashMap<String, String>>) -> impl Responder {
    if let Some(code) = req.get("code") {
        let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
        let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
        let redirect_uri = "http://localhost:1500/api/auth/discord/redirect";

        let mut params = std::collections::HashMap::new();
        params.insert("client_id", client_id);
        params.insert("client_secret", client_secret);
        params.insert("grant_type", "authorization_code".to_string());
        params.insert("code", code.clone());
        params.insert("redirect_uri", redirect_uri.to_string());

        let client = reqwest::Client::new();
        let output = client
            .post("https://discord.com/api/oauth2/token")
            .form(&params)
            .send()
            .await
            .expect("Failed to send request")
            .json::<serde_json::Value>()
            .await
            .expect("Failed to parse response");

        if let Some(access) = output.get("access_token").and_then(|v| v.as_str()) {
            let access = access.to_string();
            let connections: Vec<Connection> = client
                .get("https://discord.com/api/v10/users/@me/connections")
                .header("Authorization", format!("Bearer {}", access))
                .send()
                .await
                .expect("Failed to send request")
                .json::<Vec<Connection>>()
                .await
                .expect("Failed to parse response");
            let user: User = client
                .get("https://discord.com/api/v10/users/@me")
                .header("Authorization", format!("Bearer {}", access))
                .send()
                .await
                .expect("Failed to send request")
                .json::<User>()
                .await
                .expect("Failed to parse response");

            if let Some(twitch_connection) = connections
                .iter()
                .find(|conn| conn.connection_type == "twitch")
            {
                println!("{}", twitch_connection.name);
                println!("{}", user.id);
                match connect_database() {
                    Ok(conn) => match user_exists(&conn, user.id.into()) {
                        Ok(exists) => {
                            if exists {
                                let _ = insert_twitch_tag(user.id.into(), &twitch_connection.name);
                            } else {
                                let _ = insert_user(&conn, user.id.into(), &user.name);
                                let _ = insert_twitch_tag(user.id.into(), &twitch_connection.name);
                            }
                        }
                        Err(e) => {
                            eprintln!("Error checking if user exists: {}", e);
                        }
                    },
                    Err(conn_err) => {
                        eprintln!("Error connecting db{}", conn_err)
                    }
                }
            } else {
                println!("No Twitch connection found");
            }
        }
    }
    HttpResponse::Ok()
}
