use crate::{responses, utils::numbers, Context, Error};
use rand::Rng;

/// Rolls dice (subcommand required)
///
/// (PREFIX | SLASH) roll <subcommand>
#[poise::command(
    prefix_command,
    slash_command,
    subcommands("d4", "d6", "d8", "d10", "d12", "d20", "d100"),
    subcommand_required,
    category = "D&D"
)]
pub async fn roll(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Rolls a d4
#[poise::command(prefix_command, slash_command)]
pub async fn d4(ctx: Context<'_>) -> Result<(), Error> {
    roll_and_reply(ctx, 4).await
}

/// Rolls a d6
#[poise::command(prefix_command, slash_command)]
pub async fn d6(ctx: Context<'_>) -> Result<(), Error> {
    roll_and_reply(ctx, 6).await
}

/// Rolls a d8
#[poise::command(prefix_command, slash_command)]
pub async fn d8(ctx: Context<'_>) -> Result<(), Error> {
    roll_and_reply(ctx, 8).await
}

/// Rolls a d10
#[poise::command(prefix_command, slash_command)]
pub async fn d10(ctx: Context<'_>) -> Result<(), Error> {
    roll_and_reply(ctx, 10).await
}

/// Rolls a d12
#[poise::command(prefix_command, slash_command)]
pub async fn d12(ctx: Context<'_>) -> Result<(), Error> {
    roll_and_reply(ctx, 12).await
}

/// Rolls a d20
#[poise::command(prefix_command, slash_command)]
pub async fn d20(ctx: Context<'_>) -> Result<(), Error> {
    roll_and_reply(ctx, 20).await
}

/// Rolls a d100
#[poise::command(prefix_command, slash_command)]
pub async fn d100(ctx: Context<'_>) -> Result<(), Error> {
    roll_and_reply(ctx, 100).await
}

async fn roll_and_reply(ctx: Context<'_>, amount: i64) -> Result<(), Error> {
    let mut result = rand::thread_rng().gen_range(1..=amount);

    if amount == 100 {
        result = numbers::round_to_nearest_10(result).await;
    }

    responses::success(ctx, &format!("You rolled a {}.", result)).await
}
