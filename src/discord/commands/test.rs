use poise::serenity_prelude::{
    collector, CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage,
};

use crate::data_structs::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn poker_verify(ctx: Context<'_>) -> Result<(), Error> {
    let pages = [
        CreateEmbed::default()
            .title("Step 1")
            .description("Some descriptive text for page 1")
            .image("https://cdn2.thecatapi.com/images/4-GdyX_fu.jpg"),
        // Page 2
        CreateEmbed::default().title("Step 2").field(
            "Important Field",
            "Value of the field",
            false,
        ),
        CreateEmbed::default()
            .title("Step 3")
            .description("Some descriptive text for page 1")
            .image("https://cdn2.thecatapi.com/images/NoQGHgPl7.jpg"),
        // Page 2
        CreateEmbed::default().title("Step 4").field(
            "Important Field",
            "Value of the field",
            false,
        ),
    ];
    // Define some unique identifiers for the navigation buttons
    let ctx_id = ctx.id();
    let prev_button_id = format!("{}prev", ctx_id);
    let next_button_id = format!("{}next", ctx_id);

    // Send the embed with the first page as content
    let reply = {
        let mut components = Vec::new();

        if pages.len() > 1 {
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
