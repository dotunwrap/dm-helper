use super::settings::get_settings;
use crate::{Context, Error};

pub async fn has_dnd_role(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx
        .author_member()
        .await
        .unwrap()
        .roles
        .contains(&get_settings(ctx).await.unwrap_or_default().dnd_role_id))
}

pub async fn has_dm_role(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx
        .author_member()
        .await
        .unwrap()
        .roles
        .contains(&get_settings(ctx).await.unwrap_or_default().dm_role_id))
}
