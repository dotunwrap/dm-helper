use crate::ApplicationContext;
use crate::{utils::db, Context, Error};
use mysql::prelude::*;
use mysql::*;
use poise::serenity_prelude as serenity;
use std::num::NonZeroU64;

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

#[derive(Debug, poise::Modal)]
#[name = "Settings"]
struct SettingsModal {
    #[name = "D&D Role ID"]
    #[min_length = 17]
    #[max_length = 20]
    dnd_role_id: String,
    #[name = "DM Role ID"]
    #[min_length = 17]
    #[max_length = 20]
    dm_role_id: String,
}

/// Configures the settings for the server
#[poise::command(slash_command, guild_only, category = "Settings")]
pub async fn settings(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    use poise::Modal as _;

    let settings = get_settings(poise::Context::Application(ctx)).await;

    let data: SettingsModal = if settings.is_some() {
        SettingsModal::execute_with_defaults(
            ctx,
            SettingsModal {
                dnd_role_id: settings.as_ref().unwrap().dnd_role_id.to_string(),
                dm_role_id: settings.as_ref().unwrap().dm_role_id.to_string(),
            },
        )
        .await?
        .expect("Failed to execute the modal with defaults.")
    } else {
        SettingsModal::execute(ctx).await?.unwrap()
    };

    let dnd_role_id = match data.dnd_role_id.parse::<NonZeroU64>() {
        Ok(dnd_role_id) => serenity::RoleId::from(dnd_role_id),
        Err(_) => serenity::RoleId::from(NonZeroU64::MIN),
    };

    if dnd_role_id.get() == u64::from(NonZeroU64::MIN) {
        ctx.reply("Role IDs cannot be zero.").await?;
        return Ok(());
    }

    if ctx.guild().unwrap().roles.get(&dnd_role_id).is_none() {
        ctx.reply(format!(
            "Role ID `{}` does not exist and cannot be used for the D&D role.",
            dnd_role_id
        ))
        .await?;
        return Ok(());
    }

    let dm_role_id = match data.dm_role_id.parse::<NonZeroU64>() {
        Ok(dm_role_id) => serenity::RoleId::from(dm_role_id),
        Err(_) => serenity::RoleId::from(NonZeroU64::MIN),
    };

    if dm_role_id.get() == u64::from(NonZeroU64::MIN) {
        ctx.reply("Role IDs cannot be zero.").await?;
        return Ok(());
    }

    if ctx.guild().unwrap().roles.get(&dm_role_id).is_none() {
        ctx.reply(format!(
            "Role ID `{}` does not exist and cannot be used for the DM role.",
            dm_role_id
        ))
        .await?;
        return Ok(());
    }

    if settings.is_some()
        && dm_role_id == settings.as_ref().unwrap().dm_role_id
        && dnd_role_id == settings.unwrap().dnd_role_id
    {
        ctx.reply("No changes were made.").await?;
        return Ok(());
    }

    ctx.data().db.get_conn().expect("Failed to get connection")
        .exec_drop(
            "INSERT INTO settings (
                guild_id,
                dnd_role_id,
                dm_role_id
            ) VALUES (
                :guild_id,
                :dnd_role_id,
                :dm_role_id
            )
            ON DUPLICATE KEY UPDATE
                dnd_role_id = IF(VALUES(dnd_role_id) != dnd_role_id, VALUES(dnd_role_id), dnd_role_id),
                dm_role_id = IF(VALUES(dm_role_id) != dm_role_id, VALUES(dm_role_id), dm_role_id)",
            params! { "guild_id" => ctx.guild_id().unwrap().get(), "dnd_role_id" => dnd_role_id.get(), "dm_role_id" => dm_role_id.get() },
        )
        .expect("Failed to insert settings");

    ctx.reply("Settings configured.").await?;

    Ok(())
}

pub async fn get_settings(ctx: Context<'_>) -> Option<Settings> {
    let guild_id = ctx.guild_id();

    if guild_id.is_none() {
        return None;
    }

    db::get_db_conn(ctx)
        .exec_first::<Settings, _, _>(
            "SELECT guild_id, dnd_role_id, dm_role_id FROM settings WHERE guild_id = :guild_id",
            params! { "guild_id" => guild_id.unwrap().get() },
        )
        .expect("Failed to get settings")
}

pub async fn does_guild_have_settings(ctx: Context<'_>) -> bool {
    db::get_db_conn(ctx)
        .exec_first::<i64, _, _>(
            "SELECT id FROM settings WHERE guild_id = :guild_id",
            params! { "guild_id" => ctx.guild_id().unwrap().to_string() },
        )
        .expect("Failed to check if settings exist")
        .is_some()
}
