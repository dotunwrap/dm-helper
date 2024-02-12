use crate::utils::db;
use crate::Context;
use futures::{Stream, StreamExt};
use mysql::prelude::*;
use mysql::*;
use poise::serenity_prelude as serenity;

pub mod session;

// TODO: Centralize the code to "get_guild_id" where it returns serenity::GuildId::default() on
// a None value for ctx.guild_id() to avoid redundancy

pub async fn autocomplete_campaign<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let guild_id = if let Some(guild_id) = ctx.guild_id() {
        guild_id
    } else {
        serenity::GuildId::default()
    };

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

pub async fn get_id_from_name(ctx: Context<'_>, name: String) -> i64 {
    let guild_id = if let Some(guild_id) = ctx.guild_id() {
        guild_id
    } else {
        serenity::GuildId::default()
    };

    db::get_db_conn(ctx)
        .exec_first(
            "SELECT id FROM campaigns WHERE name LIKE :name AND guild_id = :guild_id",
            params! { name, "guild_id" => guild_id.get() },
        )
        .expect("Failed to get ID from name")
        .expect("Campaign not found")
}

pub async fn get_name_from_id(ctx: Context<'_>, id: i64) -> String {
    let guild_id = if let Some(guild_id) = ctx.guild_id() {
        guild_id
    } else {
        serenity::GuildId::default()
    };

    db::get_db_conn(ctx)
        .exec_first(
            "SELECT name FROM campaigns WHERE id = :id AND guild_id = :guild_id",
            params! { id, "guild_id" => guild_id.get() },
        )
        .expect("Failed to get name from ID")
        .expect("Campaign not found")
}

pub async fn does_campaign_exist(ctx: Context<'_>, id: i64) -> bool {
    let guild_id = if let Some(guild_id) = ctx.guild_id() {
        guild_id
    } else {
        serenity::GuildId::default()
    };

    db::get_db_conn(ctx)
        .exec_first::<i64, _, _>(
            "SELECT id FROM campaigns WHERE id = :id AND guild_id = :guild_id",
            params! { id, "guild_id" => guild_id.get() },
        )
        .expect("Failed to check if campaign exists")
        .is_some()
}
