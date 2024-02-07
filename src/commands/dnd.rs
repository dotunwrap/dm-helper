use crate::{responses, Context, Error};
use poise::serenity_prelude as serenity;

pub mod campaign;
pub mod dice;

pub async fn has_dnd_role(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx
        .author_member()
        .await
        .unwrap()
        .roles
        .contains(&serenity::RoleId::from(ctx.data().dnd_role)))
}

pub async fn dnd_check(ctx: Context<'_>) -> Result<bool, Error> {
    if !has_dnd_role(ctx).await.unwrap() {
        responses::invalid_permissions(ctx).await.unwrap();
        return Ok(false);
    }

    Ok(true)
}
