use super::users;
use crate::{responses::invalid_permissions, Context, Error};

pub async fn dnd_check(ctx: Context<'_>) -> Result<bool, Error> {
    if !users::has_dnd_role(ctx).await? {
        invalid_permissions(ctx).await?;
        return Ok(false);
    }

    Ok(true)
}

pub async fn dm_check(ctx: Context<'_>) -> Result<bool, Error> {
    if !users::has_dm_role(ctx).await? {
        invalid_permissions(ctx).await?;
        return Ok(false);
    }

    Ok(true)
}
