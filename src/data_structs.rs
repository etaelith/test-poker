use serde::{Deserialize, Serialize};
pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseStatus {
    pub success: bool,
    #[serde(rename = "description")]
    pub success_description: Option<String>,
    #[serde(rename = "error_message")]
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopTen {
    pub name: String,
    pub position: u8,
    pub points: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tournaments {
    pub tournament_date: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Connection {
    #[serde(rename = "type")]
    pub connection_type: String,
    pub id: String,
    pub name: String,
}
#[derive(Debug, poise::Modal)]
#[allow(dead_code)] // fields only used for Debug print
pub struct VerifyBitmex {
    pub api_key: String,
    pub api_secret: String,
}
