mod auth;
mod data_structs;
mod db;
mod discord;

use actix_web::{App, HttpServer};
use discord::setup_discord;

use std::env;

use crate::auth::redirect;

#[actix_web::main]
async fn main() {
    let discord_handle = setup_discord().await;

    dotenv::dotenv().ok();
    env_logger::init();

    let host = "0.0.0.0";
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(1500);

    let server = HttpServer::new(|| App::new().service(redirect))
        .bind((host, port))
        .unwrap()
        .run();

    println!("Starting server at http://{}:{}", host, port);
    tokio::select! {
        _ = server => {},
        _ = discord_handle => {}
    }
}
