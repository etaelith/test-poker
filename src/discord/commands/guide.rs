use poise::serenity_prelude::{
    collector, CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage,
};

use crate::data_structs::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn poker_verify(ctx: Context<'_>) -> Result<(), Error> {
    let footer = CreateEmbedFooter::new(
        "Recorda que una manera de mantener mas segura tu cuenta es tener activado el 2FA",
    );
    let pages = [
        CreateEmbed::default()
            .title("Primer paso")
            .description("Para registrarte tenes que tener creada la cuenta con el siguiente link o con el de alguno de los amigos de la casa!")
            .url("https://www.bitmex.com/app/register/gzAfFs")
            .image("https://cdn2.thecatapi.com/images/4-GdyX_fu.jpg").footer(footer.clone()),
        // Page 2
        CreateEmbed::default().title("Segundo paso").field(
            "Recorda que necesitas validar KYC(Know Your Customer) y elegir un nombre en la guild a la que perteneces",
            "Para elegir el nombre simplemente tenes que ir a la barra inferior en bitmex y poner el nombre que deseas",
            false,
        ).image("https://res.cloudinary.com/etaelith/image/upload/v1710711356/imagen_2_n576bm.png").footer(footer.clone()),
        // page 3
        CreateEmbed::default()
            .title("Tercer paso")
            .description("Dirigirte a la pestaña de API KEYS")
            .image("https://res.cloudinary.com/etaelith/image/upload/v1710711397/imagen_3_kagcov.png").footer(footer.clone()),
        // Page 4
        CreateEmbed::default().title("Cuarto Paso").description("Creacion de una API KEY de solo lectura.\n Simplemente vas a tener que darle al boton de Create API Key").image("https://res.cloudinary.com/etaelith/image/upload/v1710711471/imagen_4_kwjptt.png").footer(footer.clone()),
        // Page 5
        CreateEmbed::default().title("Cuarto Paso").description("Chequeo de que este todo OK para verificar").image("https://res.cloudinary.com/etaelith/image/upload/v1710711475/imagen_5_esdidm.png").footer(footer.clone()),
        CreateEmbed::default().title("Quinto Paso").description("Ahora usa en este mismo chat el comando /poker_verify_bitmex y coloca el ID y el SECRET donde se indica y espera la verificacion").image("https://cdn2.thecatapi.com/images/4-GdyX_fu.jpg").footer(footer.clone()),
    ];
    // Define some unique identifiers for the navigation buttons
    let ctx_id = ctx.id();
    let prev_button_id = format!("{}prev", ctx_id);
    let next_button_id = format!("{}next", ctx_id);

    // Send the embed with the first page as content
    let reply = {
        let mut components = Vec::new();

        if pages.len() > 1 {
            components.push(
                CreateButton::new_link("https://www.bitmex.com/app/register/gzAfFs")
                    .label("Registrarse"),
            );
            // If there are more than one page, add a "Next" button
            components.push(CreateButton::new(&next_button_id).emoji('▶').label("Next"));
        }

        CreateMessage::default()
            .embed(pages[0].clone())
            .components(vec![CreateActionRow::Buttons(components)])
    };

    ctx.author().dm(ctx.http(), reply).await?;

    // Loop through incoming interactions with the navigation buttons
    let mut current_page = 0;
    while let Some(press) = collector::ComponentInteractionCollector::new(ctx)
        // We defined our button IDs to start with `ctx_id`. If they don't, some other command's
        // button was pressed
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        // Timeout when no navigation button has been pressed for 24 hours
        .timeout(std::time::Duration::from_secs(3600 * 24))
        .await
    {
        // Depending on which button was pressed, go to next or previous page
        if press.data.custom_id == next_button_id {
            current_page += 1;
            if current_page >= pages.len() {
                current_page = 0;
            }
        } else if press.data.custom_id == prev_button_id {
            current_page = current_page.checked_sub(1).unwrap_or(pages.len() - 1);
        } else {
            // This is an unrelated button interaction
            continue;
        }

        let mut components = Vec::new();
        if current_page > 0 {
            // If not on the first page, add a "Previous" button
            components.push(CreateButton::new(&prev_button_id).emoji('◀').label("Back"));
        }
        if current_page < pages.len() - 1 {
            // If not on the last page, add a "Next" button
            components.push(CreateButton::new(&next_button_id).emoji('▶').label("Next"));
        }

        // Update the message with the new page contents and appropriate navigation buttons
        press
            .create_response(
                ctx.serenity_context(),
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .embed(pages[current_page].clone())
                        .components(vec![CreateActionRow::Buttons(components)]),
                ),
            )
            .await?;
    }

    Ok(())
}
