use super::{db, guilds::get_guild_id};
use crate::Context;
use mysql::prelude::*;
use mysql::*;

pub async fn does_session_exist(ctx: Context<'_>, session_id: i64) -> bool {
    let guild_id = get_guild_id(ctx).await;

    db::get_db_conn(ctx)
        .exec_first::<i64, _, _>(
            "SELECT id FROM sessions WHERE id = :session_id AND campaign_id IN (SELECT id FROM campaigns WHERE guild_id = :guild_id)",
            params! { session_id, "guild_id" => guild_id.get() },
        )
        .expect("Failed to check if session exists")
        .is_some()
}
