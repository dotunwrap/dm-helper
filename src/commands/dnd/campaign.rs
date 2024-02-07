use crate::utils::db;
use crate::Context;
use futures::{Stream, StreamExt};
use mysql::prelude::*;
use mysql::*;

pub mod session;

pub struct Campaign {
    _id: i64,
    _dm_id: String,
    _name: String,
    _description: String,
}

impl Campaign {
    fn _new(_id: i64, _dm_id: String, _name: String, _description: String) -> Self {
        Self {
            _id,
            _dm_id,
            _name,
            _description,
        }
    }
}

pub async fn autocomplete_campaign<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let campaigns: Vec<String> = db::get_db_conn(ctx)
        .query("SELECT DISTINCT name FROM campaigns")
        .unwrap();
    futures::stream::iter(campaigns)
        .filter(move |c| futures::future::ready(c.starts_with(partial)))
        .map(|c| c.to_string())
}

pub async fn get_id_from_name(ctx: Context<'_>, name: String) -> i64 {
    db::get_db_conn(ctx)
        .exec_first(
            "SELECT id FROM campaigns WHERE name LIKE :name",
            params! { name },
        )
        .expect("Failed to get ID from name")
        .expect("Campaign not found")
}

pub async fn get_name_from_id(ctx: Context<'_>, id: i64) -> String {
    db::get_db_conn(ctx)
        .exec_first("SELECT name FROM campaigns WHERE id = :id", params! { id })
        .expect("Failed to get name from ID")
        .expect("Campaign not found")
}

pub async fn does_campaign_exist(ctx: Context<'_>, id: i64) -> bool {
    db::get_db_conn(ctx)
        .exec_first::<i64, _, _>("SELECT id FROM campaigns WHERE id = :id", params! { id })
        .expect("Failed to check if campaign exists")
        .is_some()
}
