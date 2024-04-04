use crate::models::{NewSession, Session};
use crate::utils::db::get_conn;
use crate::Context;
use diesel::prelude::*;

pub fn create_session(ctx: Context<'_>, session: NewSession) {
    use crate::schema::sessions::dsl::*;

    let conn = &mut get_conn(ctx);

    diesel::insert_into(sessions)
        .values(&session)
        .execute(conn)
        .expect("Error saving new session");
}

pub fn update_session(ctx: Context<'_>, session: Session) {
    use crate::schema::sessions::dsl::*;

    let conn = &mut get_conn(ctx);

    diesel::update(sessions.find(session.id))
        .set(&session)
        .execute(conn)
        .expect("Error updating session");
}

pub fn bulk_cancel_sessions(ctx: Context<'_>, guild_id_i64: i64) {
    use crate::schema::sessions::dsl::*;

    let campaign_ids = match super::campaign_ops::get_campaigns(ctx, guild_id_i64) {
        Some(campaigns) => campaigns
            .iter()
            .map(|campaign| campaign.id)
            .collect::<Vec<i32>>(),
        None => return,
    };

    let conn = &mut get_conn(ctx);

    campaign_ids.into_iter().for_each(|campaign_id_i32| {
        diesel::update(sessions.filter(campaign_id.eq(campaign_id_i32)))
            .set(status.eq(2))
            .execute(conn)
            .expect("Error updating session");
    });
}

pub fn get_sessions(ctx: Context<'_>, campaign_id_i32: i32) -> Option<Vec<Session>> {
    use crate::schema::sessions::dsl::*;

    let conn = &mut get_conn(ctx);

    sessions
        .filter(campaign_id.eq(campaign_id_i32))
        .load::<Session>(conn)
        .ok()
}

pub fn get_session(ctx: Context<'_>, session_id_i32: i32) -> Option<Session> {
    use crate::schema::sessions::dsl::*;

    let conn = &mut get_conn(ctx);

    sessions
        .filter(id.eq(session_id_i32))
        .first::<Session>(conn)
        .ok()
}

pub fn does_session_exist(ctx: Context<'_>, session_id_i32: i32) -> bool {
    use crate::schema::sessions::dsl::*;

    let conn = &mut get_conn(ctx);

    sessions
        .filter(id.eq(session_id_i32))
        .first::<Session>(conn)
        .is_ok()
}
