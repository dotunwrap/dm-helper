use crate::{
    responses,
    utils::{autocompletes::autocomplete_campaign, campaigns, checks, db, guilds::get_guild_id},
    Context, Error,
};
use mysql::params;
use mysql::prelude::*;
use poise::serenity_prelude as serenity;

pub mod session;

/// D&D Campaigns (subcommand required)
#[poise::command(
    slash_command,
    subcommands("create", "edit", "delete", "list"),
    subcommand_required,
    check = "checks::dnd_check",
    category = "D&D"
)]
pub async fn campaign(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Creates a new D&D campaign (DMs only)
#[poise::command(slash_command, check = "checks::dm_check")]
pub async fn create(
    ctx: Context<'_>,
    #[description = "The name of the campaign"] name: String,
    #[description = "The description of the campaign"] description: Option<String>,
    #[description = "The user that is DMing the campaign. Defaults to you"] dm: Option<
        serenity::User,
    >,
) -> Result<(), Error> {
    ctx.defer().await?;

    if campaigns::is_campaign_name_taken(ctx, &name).await {
        return responses::failure(ctx, &format!("Campaign with name {} already exists.", name))
            .await;
    }

    let guild_id = get_guild_id(ctx).await;
    let dm_id = match dm {
        Some(dm) => dm.id.get(),
        None => ctx.author().id.get(),
    };

    db::get_db_conn(ctx).exec_drop(
        "INSERT INTO campaigns (
            guild_id, dm_id, name, description
        ) VALUES (
            :guild_id, :dm_id, :name, :description
        )",
        params! {
            "guild_id" => guild_id.get(),
            dm_id,
            name,
            description
        },
    )?;

    responses::success(ctx, "Campaign created.").await
}

/// Edits an existing D&D campaign (DMs only)
#[poise::command(
    slash_command,
    subcommands("name", "description", "dm"),
    subcommand_required,
    check = "checks::dm_check"
)]
pub async fn edit(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Edits the name of an existing D&D campaign (DMs only)
#[poise::command(slash_command)]
pub async fn name(
    ctx: Context<'_>,
    #[description = "The name of the campaign to edit"]
    #[autocomplete = "autocomplete_campaign"]
    old_name: String,
    #[description = "The new name of the campaign"] new_name: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    if !campaigns::does_campaign_exist(ctx, &old_name).await {
        return responses::failure(
            ctx,
            &format!("Campaign with name {} does not exist.", old_name),
        )
        .await;
    }

    if campaigns::is_campaign_name_taken(ctx, &new_name).await {
        return responses::failure(
            ctx,
            &format!("Campaign with name {} already exists.", new_name),
        )
        .await;
    }

    let guild_id = get_guild_id(ctx).await;

    db::get_db_conn(ctx).exec_drop(
        "UPDATE campaigns SET name = :new_name WHERE name = :old_name AND guild_id = :guild_id",
        params! {
            "new_name" => &new_name,
            "old_name" => &old_name,
            "guild_id" => guild_id.get()
        },
    )?;

    responses::success(
        ctx,
        &format!("Campaign {} updated to {}.", old_name, new_name),
    )
    .await
}

/// Edits the description of an existing D&D campaign (DMs only)
#[poise::command(slash_command)]
pub async fn description(
    ctx: Context<'_>,
    #[description = "The name of the campaign to edit"]
    #[autocomplete = "autocomplete_campaign"]
    name: String,
    #[description = "The new description of the campaign"] description: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    if !campaigns::does_campaign_exist(ctx, &name).await {
        return responses::failure(ctx, &format!("Campaign with name {} does not exist.", name))
            .await;
    }

    let guild_id = get_guild_id(ctx).await;

    db::get_db_conn(ctx).exec_drop(
        "UPDATE campaigns SET description = :description WHERE name = :name AND guild_id = :guild_id",
        params! {
            description,
            "name" => &name,
            "guild_id" => guild_id.get()
        }
    )?;

    responses::success(ctx, &format!("Campaign {}'s description updated.", name)).await
}

/// Edits the DM of an existing D&D campaign (subcommand required) (DMs only)
#[poise::command(slash_command)]
pub async fn dm(
    ctx: Context<'_>,
    #[description = "The name of the campaign to edit"]
    #[autocomplete = "autocomplete_campaign"]
    name: String,
    #[description = "The new DM of the campaign"] dm: serenity::User,
) -> Result<(), Error> {
    ctx.defer().await?;

    if !campaigns::does_campaign_exist(ctx, &name).await {
        return responses::failure(ctx, &format!("Campaign with name {} does not exist.", name))
            .await;
    }

    let guild_id = get_guild_id(ctx).await;

    db::get_db_conn(ctx).exec_drop(
        "UPDATE campaigns SET dm_id = :dm_id WHERE name = :name AND guild_id = :guild_id",
        params! {
            "dm_id" => dm.id.get(),
            "name" => &name,
            "guild_id" => guild_id.get()
        },
    )?;

    responses::success(
        ctx,
        &format!("Campaign {}'s DM updated to <@{}>.", name, dm.id),
    )
    .await
}

/// Deletes an existing D&D campaign (DMs only)
#[poise::command(slash_command)]
pub async fn delete(
    ctx: Context<'_>,
    #[description = "The name of the campaign to delete"]
    #[autocomplete = "autocomplete_campaign"]
    name: String,
) -> Result<(), Error> {
    async fn delete_campaign(ctx: Context<'_>, name: String) -> Result<(), Error> {
        ctx.defer().await?;

        if !campaigns::does_campaign_exist(ctx, &name).await {
            return responses::failure(
                ctx,
                &format!("Campaign with name {} does not exist.", name),
            )
            .await;
        }

        let guild_id = get_guild_id(ctx).await;

        db::get_db_conn(ctx).exec_drop(
            "DELETE FROM campaigns WHERE name = :name AND guild_id = :guild_id",
            params! {
                "name" => &name,
                "guild_id" => guild_id.get()
            },
        )?;

        responses::success(ctx, &format!("Campaign {} deleted.", name)).await
    }

    responses::verify_command(ctx, &delete_campaign, name).await
}

/// Lists all D&D campaigns
#[poise::command(slash_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let guild_id = get_guild_id(ctx).await;
    let mut embeds: Vec<serenity::CreateEmbed> = vec![];

    db::get_db_conn(ctx).exec_map(
        "SELECT name, description, dm_id FROM campaigns WHERE guild_id = :guild_id",
        params! {
            "guild_id" => guild_id.get()
        },
        |(name, description, dm_id): (String, Option<String>, u64)| {
            embeds.push(
                serenity::CreateEmbed::new()
                    .title(&name)
                    .description(match description {
                        Some(description) => description,
                        None => String::from(""),
                    })
                    .field("DM", format!("<@{}>", dm_id), false),
            )
        },
    )?;

    responses::paginate_embeds(ctx, embeds).await
}
