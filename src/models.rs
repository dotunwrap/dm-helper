use crate::schema::{campaigns, characters, responses, sessions, settings};

#[derive(Insertable)]
#[diesel(table_name = campaigns)]
pub struct NewCampaign<'a> {
    pub guild_id: i64,
    pub dm_id: i64,
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub link: Option<&'a str>,
    pub deleted: bool,
    pub created_date: chrono::NaiveDateTime,
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct Campaign {
    pub id: i32,
    pub guild_id: i64,
    pub dm_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub link: Option<String>,
    pub deleted: bool,
    pub created_date: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = characters)]
pub struct NewCharacter<'a> {
    pub campaign_id: i32,
    pub player_id: i64,
    pub name: &'a str,
    pub race: &'a str,
    pub class: &'a str,
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct Character {
    pub id: i32,
    pub campaign_id: i32,
    pub player_id: i64,
    pub name: String,
    pub race: String,
    pub class: String,
}

#[derive(Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession<'a> {
    pub campaign_id: i32,
    pub author_id: i64,
    pub location: Option<&'a str>,
    pub status: i16,
    pub created_date: chrono::NaiveDateTime,
    pub scheduled_date: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct Session {
    pub id: i32,
    pub campaign_id: i32,
    pub author_id: i64,
    pub location: Option<String>,
    pub status: i16,
    pub created_date: chrono::NaiveDateTime,
    pub scheduled_date: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = responses)]
pub struct NewResponse {
    pub session_id: i32,
    pub respondee_id: i64,
    pub response: i16,
    pub responded_date: chrono::NaiveDateTime,
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct Response {
    pub id: i32,
    pub session_id: i32,
    pub respondee_id: i64,
    pub response: i16,
    pub responded_date: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = settings)]
pub struct NewSetting {
    pub guild_id: i64,
    pub dnd_role_id: Option<i64>,
    pub dm_role_id: Option<i64>,
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct Setting {
    pub guild_id: i64,
    pub dnd_role_id: Option<i64>,
    pub dm_role_id: Option<i64>,
}
