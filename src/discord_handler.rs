use ::serenity::model::event::ResumedEvent;
use ::serenity::model::gateway::Ready;
use ::serenity::prelude::*;
use serenity::async_trait;

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
