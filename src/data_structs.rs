use serde::{Deserialize, Serialize};

pub struct Data {} // User data, which is stored and accessible in all command invocations

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
