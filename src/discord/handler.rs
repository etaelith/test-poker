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
                        let msg = msg.channel_id.send_message(&ctx.http, builder).await;
                        if let Err(why) = msg {
                            println!("Error sending message: {:?}", why);
                        }
                    } else {
                        let builder =
                            CreateMessage::new().content("Error al obtener el top 10 de usuarios");
                        let msg = msg.channel_id.send_message(&ctx.http, builder).await;
                        if let Err(why) = msg {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                }
                Err(err) => {
                    println!("Error al conectar a la base de datos: {:?}", err);
                }
            }
        }
        /*  match get_last_tournament() {
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
                       let embed = CreateEmbed::new().title("Último Torneo").fields(fields_vec);

                       let builder = CreateMessage::new().embed(embed);
                       let msg = msg.channel_id.send_message(&ctx.http, builder).await;
                       if let Err(why) = msg {
                           println!("Error sending message: {:?}", why);
                       }
                   } else {
                       let builder = CreateMessage::new().content("Error al obtener el último torneo");
                       let msg = msg.channel_id.send_message(&ctx.http, builder).await;
                       if let Err(why) = msg {
                           println!("Error sending message: {:?}", why);
                       }
                   }
               }
               Err(err) => {
                   println!("Error al conectar a la base de datos: {:?}", err);
               }
           }
        */
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Bot resumed");
    }
}
