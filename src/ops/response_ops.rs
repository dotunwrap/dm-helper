use crate::models::{NewResponse, Response};
use crate::utils::db::get_conn;
use crate::Context;
use diesel::prelude::*;

pub fn create_response(ctx: Context<'_>, new_response: NewResponse) {
    use crate::schema::responses::dsl::*;

    let conn = &mut get_conn(ctx);

    diesel::insert_into(responses)
        .values(&new_response)
        .on_conflict((session_id, respondee_id))
        .do_update()
        .set(&new_response)
        .execute(conn)
        .expect("Error saving new response");
}

pub fn update_response(ctx: Context<'_>, updated_response: Response) {
    use crate::schema::responses::dsl::*;

    let conn = &mut get_conn(ctx);

    diesel::update(responses.find(updated_response.id))
        .set(&updated_response)
        .execute(conn)
        .expect("Error updating response");
}

pub fn get_responses_for_session(ctx: Context<'_>, session_id_i32: i32) -> Option<Vec<Response>> {
    use crate::schema::responses::dsl::*;

    let conn = &mut get_conn(ctx);

    responses
        .filter(session_id.eq(session_id_i32))
        .load::<Response>(conn)
        .ok()
}
