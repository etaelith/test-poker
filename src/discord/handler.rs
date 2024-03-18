use poise::serenity_prelude::model::{event::ResumedEvent, gateway::Ready};
use poise::serenity_prelude::{
    async_trait, Context, CreateEmbed, CreateMessage, EventHandler, Message,
};

use crate::data_structs::TopTen;
use crate::db::commands::table_tournaments::get_top;

pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Bot connected as: {}", ready.user.name);
    }
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!top" {
            match get_top() {
                Ok(response) => {
                    if response.success {
                        let top_users: Vec<TopTen> =
                            serde_json::from_str(&response.success_description.unwrap()).unwrap();

                        let mut fields_vec = Vec::new();
                        for user in top_users {
                            fields_vec.push((
                                format!("Posición {}", user.position),
                                format!("{}: {} puntos", user.name, user.points),
                                false,
                            ));
                        }

                        let embed = CreateEmbed::new()
                            .title("Top 10 Usuarios")
                            .fields(fields_vec);

                        let builder = CreateMessage::new().embed(embed);

                        // Despinea el mensaje anterior
                        if let Ok(pinned_msgs) = msg.channel_id.pins(&ctx.http).await {
                            for pinned_msg in pinned_msgs {
                                if let Some(embeds) = pinned_msg.embeds.first() {
                                    if embeds.title == Some("Top 10 Usuarios".to_string()) {
                                        if let Err(why) = pinned_msg.unpin(&ctx.http).await {
                                            eprintln!("Error despineando el mensaje: {:?}", why);
                                        }
                                        break;
                                    }
                                }
                            }
                        }

                        match msg.channel_id.send_message(&ctx.http, builder).await {
                            Ok(new_msg) => {
                                if let Err(why) = new_msg.pin(&ctx.http).await {
                                    eprintln!("Error pineando el mensaje: {:?}", why);
                                }
                            }
                            Err(why) => {
                                eprintln!("Error enviando el mensaje: {:?}", why);
                            }
                        }
                    } else {
                        let builder =
                            CreateMessage::new().content("Error al obtener el top 10 de usuarios");
                        let msg = msg.channel_id.send_message(&ctx.http, builder).await;
                        if let Err(why) = msg {
                            eprintln!("Error enviando el mensaje: {:?}", why);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error al conectar a la base de datos: {:?}", err);
                }
            }
        }
    }
    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Bot resumed");
    }
}
