use super::guilds::get_guild_id;
use super::id::{guild_id_to_i64, i64_to_role_id};
use crate::ops::settings_ops::get_settings;
use crate::{Context, Error};

pub async fn has_dnd_role(ctx: Context<'_>) -> Result<bool, Error> {
    let guild_id = guild_id_to_i64(get_guild_id(ctx).await).await;

    match get_settings(ctx, guild_id) {
        Some(settings) => match settings.dnd_role_id {
            Some(dnd_role_id) => {
                let dnd_role_id = i64_to_role_id(dnd_role_id).await;
                Ok(ctx
                    .author_member()
                    .await
                    .unwrap()
                    .roles
                    .contains(&dnd_role_id))
            }
            None => Ok(false),
        },
        None => Ok(false),
    }
}

pub async fn has_dm_role(ctx: Context<'_>) -> Result<bool, Error> {
    let guild_id = guild_id_to_i64(get_guild_id(ctx).await).await;

    match get_settings(ctx, guild_id) {
        Some(settings) => match settings.dm_role_id {
            Some(dm_role_id) => {
                let dm_role_id = i64_to_role_id(dm_role_id).await;
                Ok(ctx
                    .author_member()
                    .await
                    .unwrap()
                    .roles
                    .contains(&dm_role_id))
            }
            None => Ok(false),
        },
        None => Ok(false),
    }
}
