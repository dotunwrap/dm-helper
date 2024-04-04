use crate::models::{Campaign, NewCampaign};
use crate::utils::db::get_conn;
use crate::Context;
use diesel::prelude::*;

pub enum CampaignFilters {
    Name(String),
    Id(i32),
    DmId(i64),
}

pub fn create_campaign(ctx: Context<'_>, campaign: NewCampaign) {
    use crate::schema::campaigns::dsl::*;

    let conn = &mut get_conn(ctx);

    diesel::insert_into(campaigns)
        .values(&campaign)
        .execute(conn)
        .expect("Error saving new campaign");
}

pub fn update_campaign(ctx: Context<'_>, campaign: Campaign) {
    use crate::schema::campaigns::dsl::*;

    let conn = &mut get_conn(ctx);

    diesel::update(campaigns.find(campaign.id))
        .set(&campaign)
        .execute(conn)
        .expect("Error updating campaign");
}

pub fn get_campaigns(ctx: Context<'_>, guild_id_i64: i64) -> Option<Vec<Campaign>> {
    use crate::schema::campaigns::dsl::*;

    let conn = &mut get_conn(ctx);

    campaigns
        .filter(guild_id.eq(guild_id_i64))
        .filter(deleted.eq(false))
        .load::<Campaign>(conn)
        .ok()
}

pub fn get_campaign(
    ctx: Context<'_>,
    guild_id_i64: i64,
    filter: CampaignFilters,
) -> Option<Campaign> {
    use crate::schema::campaigns::dsl::*;

    let conn = &mut get_conn(ctx);

    match filter {
        CampaignFilters::Name(name_string) => campaigns
            .filter(guild_id.eq(guild_id_i64))
            .filter(name.eq(name_string))
            .filter(deleted.eq(false))
            .first::<Campaign>(conn)
            .ok(),
        CampaignFilters::Id(id_i32) => campaigns
            .filter(guild_id.eq(guild_id_i64))
            .filter(id.eq(id_i32))
            .filter(deleted.eq(false))
            .first::<Campaign>(conn)
            .ok(),
        CampaignFilters::DmId(dm_id_i64) => campaigns
            .filter(dm_id.eq(dm_id_i64))
            .filter(deleted.eq(false))
            .first::<Campaign>(conn)
            .ok(),
    }
}

pub fn get_campaign_names(ctx: Context<'_>, guild_id_i64: i64) -> Option<Vec<String>> {
    use crate::schema::campaigns::dsl::*;

    let conn = &mut get_conn(ctx);

    campaigns
        .filter(guild_id.eq(guild_id_i64))
        .filter(deleted.eq(false))
        .select(name)
        .load::<String>(conn)
        .ok()
}

pub fn get_id_from_name(ctx: Context<'_>, name_str: &str, guild_id_i64: i64) -> Option<i32> {
    use crate::schema::campaigns::dsl::*;

    let conn = &mut get_conn(ctx);

    campaigns
        .filter(name.eq(name_str))
        .filter(guild_id.eq(guild_id_i64))
        .select(id)
        .first::<i32>(conn)
        .ok()
}

pub fn get_name_from_id(ctx: Context<'_>, id_i32: i32, guild_id_i64: i64) -> Option<String> {
    use crate::schema::campaigns::dsl::*;

    let conn = &mut get_conn(ctx);

    campaigns
        .filter(id.eq(id_i32))
        .filter(guild_id.eq(guild_id_i64))
        .select(name)
        .first::<String>(conn)
        .ok()
}

pub fn does_campaign_exist(ctx: Context<'_>, name_str: &str, guild_id_i64: i64) -> bool {
    use crate::schema::campaigns::dsl::*;

    let conn = &mut get_conn(ctx);

    campaigns
        .filter(name.eq(name_str))
        .filter(guild_id.eq(guild_id_i64))
        .select(id)
        .first::<i32>(conn)
        .is_ok()
}
