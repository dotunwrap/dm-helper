use super::{db, guilds::get_guild_id};
use crate::Context;
use mysql::params;
use mysql::prelude::*;

pub async fn get_id_from_name(ctx: Context<'_>, name: &str) -> i64 {
    let guild_id = get_guild_id(ctx).await;

    db::get_db_conn(ctx)
        .exec_first(
            "SELECT id FROM campaigns WHERE name LIKE :name AND guild_id = :guild_id",
            params! { name, "guild_id" => guild_id.get() },
        )
        .expect("Failed to get ID from name")
        .expect("Campaign not found")
}

pub async fn get_name_from_id(ctx: Context<'_>, id: i64) -> String {
    let guild_id = get_guild_id(ctx).await;

    db::get_db_conn(ctx)
        .exec_first(
            "SELECT name FROM campaigns WHERE id = :id AND guild_id = :guild_id",
            params! { id, "guild_id" => guild_id.get() },
        )
        .expect("Failed to get name from ID")
        .expect("Campaign not found")
}

pub async fn does_campaign_exist(ctx: Context<'_>, name: &str) -> bool {
    let guild_id = get_guild_id(ctx).await;

    db::get_db_conn(ctx)
        .exec_first::<i64, _, _>(
            "SELECT id FROM campaigns WHERE name = :name AND guild_id = :guild_id",
            params! { name, "guild_id" => guild_id.get() },
        )
        .expect("Failed to check if campaign exists")
        .is_some()
}

pub async fn is_campaign_name_taken(ctx: Context<'_>, name: &str) -> bool {
    does_campaign_exist(ctx, name).await
}
