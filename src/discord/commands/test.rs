use poise::serenity_prelude::{
    ButtonStyle, ComponentInteractionCollector, CreateActionRow, CreateButton,
};

use crate::{
    data_structs::{Context, Error},
    discord::commands::verify::call_verify_bitmex,
};

#[derive(Debug, poise::Modal)]
#[allow(dead_code)] // fields only used for Debug print
struct VerifyBitmex {
    api_key: String,
    api_secret: String,
}

/// Tests the Modal trait with component interactions.
///
/// Should be both prefix and slash to make sure it works without any slash command interaction
/// present.
#[poise::command(prefix_command, slash_command)]
pub async fn verify_bitmex(ctx: Context<'_>) -> Result<(), Error> {
    let reply = {
        let components = vec![CreateActionRow::Buttons(vec![CreateButton::new(
            "verify_bitmex",
        )
        .label("Verify")
        .style(ButtonStyle::Success)])];

        poise::CreateReply::default()
            .ephemeral(true)
            .content("Click the button below to verify bitmex")
            .components(components)
    };

    ctx.send(reply).await?;

    while let Some(mci) = ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == "verify_bitmex")
        .await
    {
        let data =
            poise::execute_modal_on_component_interaction::<VerifyBitmex>(ctx, mci, None, None)
                .await?;
        if let Some(data) = &data {
            let api_key = &data.api_key;
            let api_secret = &data.api_secret;

            let _ = call_verify_bitmex(api_key.to_string(), api_secret.to_string()).await;
        } else {
            println!("No se recibieron datos de la interacción del componente");
        }
    }
    Ok(())
}
