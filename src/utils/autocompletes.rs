use super::{db, guilds::get_guild_id};
use crate::Context;
use futures::{Stream, StreamExt};
use mysql::params;
use mysql::prelude::*;

pub async fn autocomplete_campaign<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let guild_id = get_guild_id(ctx).await;

    let campaigns: Vec<String> = db::get_db_conn(ctx)
        .exec(
            "SELECT DISTINCT name FROM campaigns WHERE guild_id = :guild_id",
            params! { "guild_id" =>  guild_id.get() },
        )
        .unwrap();

    futures::stream::iter(campaigns)
        .filter(move |c| futures::future::ready(c.starts_with(partial)))
        .map(|c| c.to_string())
}
