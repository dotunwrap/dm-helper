use crate::{Context, Error};
use poise::serenity_prelude as serenity;

pub async fn success(ctx: Context<'_>, msg: &str) -> Result<(), Error> {
    ctx.say(msg).await?;
    Ok(())
}

pub async fn failure(ctx: Context<'_>, msg: &str) -> Result<(), Error> {
    ctx.reply(msg).await?;
    Ok(())
}

pub async fn invalid_permissions(ctx: Context<'_>) -> Result<(), Error> {
    failure(
        ctx,
        "Error: You do not have permission to use this command.",
    )
    .await
}

pub async fn paginate_embeds(
    ctx: Context<'_>,
    embeds: Vec<serenity::CreateEmbed>,
    // custom_buttons: Option<Vec<serenity::CreateButton>>,
) -> Result<(), Error> {
    if embeds.is_empty() {
        return failure(ctx, "No results found.").await;
    }

    let ctx_id = ctx.id();
    let prev_button_id = format!("{}_prev", ctx_id);
    let next_button_id = format!("{}_next", ctx_id);
    // TODO: Allow for custom buttons with custom callbacks on press
    // let mut buttons = vec![
    //     serenity::CreateButton::new(&prev_button_id).emoji("⬅".chars().next().unwrap()),
    //     serenity::CreateButton::new(&next_button_id).emoji("➡".chars().next().unwrap()),
    // ];

    // if let Some(custom_buttons) = custom_buttons {
    //     buttons.splice(1..1, custom_buttons);
    // }

    let reply = {
        let components = serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new(&prev_button_id).emoji("⬅".chars().next().unwrap()),
            serenity::CreateButton::new(&next_button_id).emoji("➡".chars().next().unwrap()),
        ]);

        poise::CreateReply::default()
            .embed(embeds[0].clone())
            .components(vec![components])
    };

    ctx.send(reply).await?;

    let mut current_page: usize = 0;
    let author_id = ctx.author().id;

    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| {
            press.user.id == author_id && press.data.custom_id.starts_with(&ctx_id.to_string())
        })
        .timeout(std::time::Duration::from_secs(180))
        .await
    {
        if press.data.custom_id == prev_button_id {
            current_page = current_page.checked_sub(1).unwrap_or(embeds.len() - 1);
        } else if press.data.custom_id == next_button_id {
            current_page += 1;
            if current_page >= embeds.len() {
                current_page = 0;
            }
        } else {
            continue;
        }

        press
            .create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::new()
                        .embed(embeds[current_page].clone()),
                ),
            )
            .await?;
    }

    Ok(())
}
