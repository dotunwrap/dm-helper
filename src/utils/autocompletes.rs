use super::{guilds::get_guild_id, id::guild_id_to_i64};
use crate::ops::campaign_ops::get_campaign_names;
use crate::Context;
use futures::{Stream, StreamExt};

pub async fn autocomplete_campaign<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let guild_id_i64 = guild_id_to_i64(get_guild_id(ctx).await).await;

    let results = match get_campaign_names(ctx, guild_id_i64) {
        Some(campaigns) => campaigns,
        None => vec![],
    };

    futures::stream::iter(results)
        .filter(move |c| futures::future::ready(c.starts_with(partial)))
        .map(|c| c.to_string())
}
