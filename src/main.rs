mod data_structs;
mod db;
mod discord;
use discord::setup_discord;

#[tokio::main]
async fn main() {
    let discord_handle = setup_discord().await;

    discord_handle.await.unwrap();
}
