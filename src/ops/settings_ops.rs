use crate::models::{NewSetting, Setting};
use crate::utils::db::get_conn;
use crate::Context;
use diesel::prelude::*;

pub fn create_settings(ctx: Context<'_>, new_settings: NewSetting) {
    use crate::schema::settings::dsl::*;

    let conn = &mut get_conn(ctx);

    diesel::insert_into(settings)
        .values(&new_settings)
        .on_conflict(guild_id)
        .do_update()
        .set(&new_settings)
        .execute(conn)
        .expect("Error saving new settings");
}

pub fn update_settings(ctx: Context<'_>, updated_settings: Setting) {
    use crate::schema::settings::dsl::*;

    let conn = &mut get_conn(ctx);

    diesel::update(settings.find(updated_settings.guild_id))
        .set(&updated_settings)
        .execute(conn)
        .expect("Error updating settings");
}

pub fn get_settings(ctx: Context<'_>, guild_id_i64: i64) -> Option<Setting> {
    use crate::schema::settings::dsl::*;

    let conn = &mut get_conn(ctx);

    settings
        .filter(guild_id.eq(guild_id_i64))
        .first::<Setting>(conn)
        .ok()
}

pub async fn does_guild_have_settings(ctx: Context<'_>, guild_id_i64: i64) -> bool {
    use crate::schema::settings::dsl::*;

    let conn = &mut get_conn(ctx);

    settings
        .filter(guild_id.eq(guild_id_i64))
        .first::<Setting>(conn)
        .is_ok()
}
