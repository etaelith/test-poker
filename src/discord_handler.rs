use poise::serenity_prelude::model::{event::ResumedEvent, gateway::Ready};
use poise::serenity_prelude::{async_trait, Context, EventHandler};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Bot connected as: {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Bot resumed");
    }
}
