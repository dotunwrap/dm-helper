use crate::Context;
use poise::serenity_prelude as serenity;

pub async fn get_guild_id(ctx: Context<'_>) -> serenity::GuildId {
    match ctx.guild_id() {
        Some(guild_id) => guild_id,
        None => serenity::GuildId::default(),
    }
}
