use super::db;
use crate::{structs::Settings, Context};
use mysql::params;
use mysql::prelude::*;

pub async fn get_settings(ctx: Context<'_>) -> Option<Settings> {
    let guild_id = ctx.guild_id();

    if guild_id.is_none() {
        return None;
    }

    db::get_db_conn(ctx)
        .exec_first::<Settings, _, _>(
            "SELECT guild_id, dnd_role_id, dm_role_id FROM settings WHERE guild_id = :guild_id",
            params! { "guild_id" => guild_id.unwrap().get() },
        )
        .expect("Failed to get settings")
}

pub async fn does_guild_have_settings(ctx: Context<'_>) -> bool {
    db::get_db_conn(ctx)
        .exec_first::<i64, _, _>(
            "SELECT id FROM settings WHERE guild_id = :guild_id",
            params! { "guild_id" => ctx.guild_id().unwrap().to_string() },
        )
        .expect("Failed to check if settings exist")
        .is_some()
}
