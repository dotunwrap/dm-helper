use crate::{
    models::NewSetting,
    ops::settings_ops,
    utils::{
        guilds::get_guild_id,
        id::{guild_id_to_i64, i64_to_role_id},
    },
    ApplicationContext, Error,
};
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
#[poise::command(
    slash_command,
    guild_only,
    category = "Settings",
    required_permissions = "MANAGE_ROLES"
)]
pub async fn settings(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    use poise::Modal as _;

    let guild_id = guild_id_to_i64(get_guild_id(poise::Context::Application(ctx)).await).await;
    let settings = settings_ops::get_settings(poise::Context::Application(ctx), guild_id);

    let data: SettingsModal = match &settings {
        Some(settings) => {
            let dnd_role_id = match settings.dnd_role_id {
                Some(dnd_role_id) => i64_to_role_id(dnd_role_id).await,
                None => serenity::RoleId::from(NonZeroU64::MIN),
            };

            let dm_role_id = match settings.dm_role_id {
                Some(dm_role_id) => i64_to_role_id(dm_role_id).await,
                None => serenity::RoleId::from(NonZeroU64::MIN),
            };

            SettingsModal::execute_with_defaults(
                ctx,
                SettingsModal {
                    dnd_role_id: dnd_role_id.to_string(),
                    dm_role_id: dm_role_id.to_string(),
                },
            )
            .await?
            .expect("Failed to execute the modal with defaults.")
        }
        None => SettingsModal::execute(ctx)
            .await?
            .expect("Failed to execute the modal."),
    };

    let dnd_role_id = match data.dnd_role_id.parse::<NonZeroU64>() {
        Ok(dnd_role_id) => serenity::RoleId::from(dnd_role_id),
        Err(_) => serenity::RoleId::from(NonZeroU64::MIN),
    };

    if dnd_role_id.get() == u64::from(NonZeroU64::MIN) {
        ctx.reply("Role IDs cannot be zero.").await?;
        return Ok(());
    }

    if !ctx.guild().unwrap().roles.contains_key(&dnd_role_id) {
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

    if !ctx.guild().unwrap().roles.contains_key(&dm_role_id) {
        ctx.reply(format!(
            "Role ID `{}` does not exist and cannot be used for the DM role.",
            dm_role_id
        ))
        .await?;
        return Ok(());
    }

    let dm_role_id = i64::try_from(dm_role_id.get()).expect("Failed to convert role ID.");
    let dnd_role_id = i64::try_from(dnd_role_id.get()).expect("Failed to convert role ID.");

    if settings.is_some()
        && dm_role_id == settings.as_ref().unwrap().dm_role_id.unwrap_or(0)
        && dnd_role_id == settings.unwrap().dnd_role_id.unwrap_or(0)
    {
        ctx.reply("No changes were made.").await?;
        return Ok(());
    }

    let settings = NewSetting {
        guild_id,
        dm_role_id: Some(dm_role_id),
        dnd_role_id: Some(dnd_role_id),
    };

    settings_ops::create_settings(poise::Context::Application(ctx), settings);

    ctx.reply("Settings configured.").await?;

    Ok(())
}
