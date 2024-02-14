use crate::{utils::settings::get_settings, ApplicationContext, Error};
use mysql::prelude::*;
use mysql::*;
use poise::serenity_prelude as serenity;
use std::num::NonZeroU64;

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
        )?;

    ctx.reply("Settings configured.").await?;

    Ok(())
}
