use crate::{responses, Context, Error};
use poise::serenity_prelude as serenity;

pub async fn author_is_staff(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx
        .author_member()
        .await
        .unwrap()
        .roles
        .contains(&serenity::RoleId::from(ctx.data().staff_role)))
}

pub async fn staff_check(ctx: Context<'_>) -> Result<bool, Error> {
    if !author_is_staff(ctx).await.unwrap() {
        responses::invalid_permissions(ctx).await.unwrap();
        return Ok(false);
    }

    Ok(true)
}
