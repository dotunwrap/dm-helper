use crate::{utils::db, Context, Error};
use mysql::prelude::*;
use mysql::*;

#[poise::command(slash_command)]
pub async fn settings(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    Ok(())
}

pub async fn does_guild_have_settings(ctx: Context<'_>) -> bool {
    db::get_db_conn(ctx)
        .exec_first::<i64, _, _>(
            "SELECT id FROM settings WHERE guild_id = :guild_id",
            params! { "guild_id" => 0 },
        )
        .expect("Failed to check if settings exist")
        .is_some()
}
