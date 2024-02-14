use mysql::prelude::FromRow;
use mysql::Row;
use poise::serenity_prelude as serenity;

pub struct Campaign {}

pub struct Character {}

pub struct Session {}

impl Session {
    pub fn translate_status(status: i64) -> String {
        match status {
            0 => String::from("Pending"),
            1 => String::from("Confirmed"),
            2 => String::from("Cancelled"),
            _ => String::from("Unknown"),
        }
    }
}

pub struct Response {
    pub id: i64,
    pub session_id: i64,
    pub respondee_id: String,
    pub response: i64,
    pub responded_date: String,
}

#[derive(Default)]
pub struct Settings {
    pub guild_id: serenity::GuildId,
    pub dnd_role_id: serenity::RoleId,
    pub dm_role_id: serenity::RoleId,
}

impl FromRow for Settings {
    fn from_row(row: Row) -> Self {
        let (guild_id, dnd_role_id, dm_role_id): (u64, u64, u64) = mysql::from_row(row);
        Settings {
            guild_id: serenity::GuildId::new(guild_id),
            dnd_role_id: serenity::RoleId::new(dnd_role_id),
            dm_role_id: serenity::RoleId::new(dm_role_id),
        }
    }

    fn from_row_opt(_row: Row) -> Result<Self, mysql::FromRowError> {
        todo!();
    }
}
