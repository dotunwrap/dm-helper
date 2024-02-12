use crate::{responses, Context, Error};

pub mod campaign;
pub mod dice;

pub async fn has_dnd_role(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx.author_member().await.unwrap().roles.contains(
        &crate::commands::settings::get_settings(ctx)
            .await
            .unwrap_or_default()
            .dnd_role_id,
    ))
}

pub async fn dnd_check(ctx: Context<'_>) -> Result<bool, Error> {
    if !has_dnd_role(ctx).await.unwrap() {
        responses::invalid_permissions(ctx).await.unwrap();
        return Ok(false);
    }

    Ok(true)
}
