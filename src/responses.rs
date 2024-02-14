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

pub async fn settings_not_configured(ctx: Context<'_>) -> Result<(), Error> {
    failure(
        ctx,
        "Settings not configured for this command.\n
        If you are an admin, please configure your settings using `/settings`.\n
        If you are not an admin, please contact one.",
    )
    .await
}

pub async fn invalid_permissions(ctx: Context<'_>) -> Result<(), Error> {
    failure(
        ctx,
        "Error: You do not have permission to use this command.",
    )
    .await
}

pub async fn verify_command<'a, T, F, Fut>(
    ctx: Context<'a>,
    function: F,
    function_arg: T,
) -> Result<(), Error>
where
    F: Fn(Context<'a>, T) -> Fut,
    Fut: futures::Future<Output = Result<(), Error>>,
{
    let ctx_id = ctx.id();
    let confirm_id = format!("{}_confirm", ctx_id);
    let cancel_id = format!("{}_cancel", ctx_id);
    let reply = {
        let components = serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new(&confirm_id).emoji("✅".chars().next().unwrap()),
            serenity::CreateButton::new(&cancel_id).emoji("❌".chars().next().unwrap()),
        ]);

        poise::CreateReply::default()
            .content("WARNING: This command is not reversible.\nAre you sure you want to continue?")
            .components(vec![components])
    };
    let author_id = ctx.author().id;

    ctx.send(reply).await?;

    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| {
            press.user.id == author_id && press.data.custom_id.starts_with(&ctx_id.to_string())
        })
        .timeout(std::time::Duration::from_secs(180))
        .await
    {
        if press.data.custom_id == confirm_id {
            press
                .create_response(
                    ctx.serenity_context(),
                    serenity::CreateInteractionResponse::UpdateMessage(
                        serenity::CreateInteractionResponseMessage::new().components(vec![]),
                    ),
                )
                .await?;
            return function(ctx, function_arg).await;
        } else if press.data.custom_id == cancel_id {
            press
                .create_response(
                    ctx.serenity_context(),
                    serenity::CreateInteractionResponse::UpdateMessage(
                        serenity::CreateInteractionResponseMessage::new().components(vec![]),
                    ),
                )
                .await?;
            return failure(ctx, "Command cancelled.").await;
        } else {
            continue;
        }
    }

    Ok(())
}

pub async fn paginate_embeds(
    ctx: Context<'_>,
    embeds: Vec<serenity::CreateEmbed>,
) -> Result<(), Error> {
    if embeds.is_empty() {
        return failure(ctx, "No results found.").await;
    }

    let ctx_id = ctx.id();
    let prev_button_id = format!("{}_prev", ctx_id);
    let next_button_id = format!("{}_next", ctx_id);
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
