use crate::{
    models::{Campaign, NewCampaign},
    ops::campaign_ops,
    responses,
    utils::{
        autocompletes::autocomplete_campaign,
        checks,
        guilds::get_guild_id,
        id::{guild_id_to_i64, user_id_to_i64},
    },
    Context, Error,
};
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
    #[description = "The URL to attach to the campaign"] link: Option<String>,
) -> Result<(), Error> {
    ctx.defer().await?;

    let guild_id = guild_id_to_i64(get_guild_id(ctx).await).await;

    if campaign_ops::does_campaign_exist(ctx, &name, guild_id) {
        return responses::failure(ctx, &format!("Campaign with name {} already exists.", name))
            .await;
    }

    let dm_id = match dm {
        Some(dm) => dm.id,
        None => ctx.author().id,
    };

    let campaign = NewCampaign {
        guild_id,
        dm_id: user_id_to_i64(dm_id).await,
        name: &name,
        description: description.as_deref(),
        link: link.as_deref(),
        deleted: false,
        created_date: chrono::Utc::now().naive_utc(),
    };

    campaign_ops::create_campaign(ctx, campaign);

    responses::success(ctx, "Campaign created.").await
}

/// Edits an existing D&D campaign (DMs only)
#[poise::command(
    slash_command,
    subcommands("name", "description", "dm", "link"),
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

    let guild_id = guild_id_to_i64(get_guild_id(ctx).await).await;

    if !campaign_ops::does_campaign_exist(ctx, &old_name, guild_id) {
        return responses::failure(
            ctx,
            &format!("Campaign with name {} does not exist.", old_name),
        )
        .await;
    }

    if campaign_ops::does_campaign_exist(ctx, &new_name, guild_id) {
        return responses::failure(
            ctx,
            &format!("Campaign with name {} already exists.", new_name),
        )
        .await;
    }

    let campaign = Campaign {
        name: new_name.clone(),
        ..campaign_ops::get_campaign(
            ctx,
            guild_id,
            campaign_ops::CampaignFilters::Name(old_name.clone()),
        )
        .unwrap()
    };

    campaign_ops::update_campaign(ctx, campaign);

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

    let guild_id = guild_id_to_i64(get_guild_id(ctx).await).await;

    if !campaign_ops::does_campaign_exist(ctx, &name, guild_id) {
        return responses::failure(ctx, &format!("Campaign with name {} does not exist.", name))
            .await;
    }

    let campaign = Campaign {
        description: Some(description),
        ..campaign_ops::get_campaign(
            ctx,
            guild_id,
            campaign_ops::CampaignFilters::Name(name.clone()),
        )
        .unwrap()
    };

    campaign_ops::update_campaign(ctx, campaign);

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

    let guild_id = guild_id_to_i64(get_guild_id(ctx).await).await;

    if !campaign_ops::does_campaign_exist(ctx, &name, guild_id) {
        return responses::failure(ctx, &format!("Campaign with name {} does not exist.", name))
            .await;
    }

    let campaign = Campaign {
        dm_id: user_id_to_i64(dm.id).await,
        ..campaign_ops::get_campaign(
            ctx,
            guild_id,
            campaign_ops::CampaignFilters::Name(name.clone()),
        )
        .unwrap()
    };

    campaign_ops::update_campaign(ctx, campaign);

    responses::success(
        ctx,
        &format!("Campaign {}'s DM updated to <@{}>.", name, dm.id),
    )
    .await
}

/// Edits the link of an existing D&D campaign (DMs only)
#[poise::command(slash_command)]
pub async fn link(
    ctx: Context<'_>,
    #[description = "The name of the campaign to edit"]
    #[autocomplete = "autocomplete_campaign"]
    name: String,
    #[description = "The new link of the campaign"] link: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    let guild_id = guild_id_to_i64(get_guild_id(ctx).await).await;

    if !campaign_ops::does_campaign_exist(ctx, &name, guild_id) {
        return responses::failure(ctx, &format!("Campaign with name {} does not exist.", name))
            .await;
    }

    let campaign = Campaign {
        link: Some(link),
        ..campaign_ops::get_campaign(
            ctx,
            guild_id,
            campaign_ops::CampaignFilters::Name(name.clone()),
        )
        .unwrap()
    };

    campaign_ops::update_campaign(ctx, campaign);

    responses::success(ctx, &format!("Campaign {}'s link updated.", name)).await
}

/// Deletes an existing D&D campaign (DMs only)
#[poise::command(slash_command, check = "checks::dm_check")]
pub async fn delete(
    ctx: Context<'_>,
    #[description = "The name of the campaign to delete"]
    #[autocomplete = "autocomplete_campaign"]
    name: String,
) -> Result<(), Error> {
    async fn delete_campaign(ctx: Context<'_>, name: String) -> Result<(), Error> {
        ctx.defer().await?;

        let guild_id = guild_id_to_i64(get_guild_id(ctx).await).await;

        if !campaign_ops::does_campaign_exist(ctx, &name, guild_id) {
            return responses::failure(
                ctx,
                &format!("Campaign with name {} does not exist.", name),
            )
            .await;
        }

        let campaign = Campaign {
            deleted: true,
            ..campaign_ops::get_campaign(
                ctx,
                guild_id,
                campaign_ops::CampaignFilters::Name(name.clone()),
            )
            .unwrap()
        };

        campaign_ops::update_campaign(ctx, campaign);

        responses::success(ctx, &format!("Campaign {} deleted.", name)).await
    }

    responses::verify_command(ctx, &delete_campaign, name).await
}

/// Lists all D&D campaigns
#[poise::command(slash_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let guild_id = guild_id_to_i64(get_guild_id(ctx).await).await;
    let mut embeds: Vec<serenity::CreateEmbed> = vec![];

    if let Some(campaigns) = campaign_ops::get_campaigns(ctx, guild_id) {
        campaigns.into_iter().for_each(|campaign| {
            embeds.push(
                serenity::CreateEmbed::new()
                    .title(&campaign.name)
                    .url(if let Some(link) = campaign.link {
                        link
                    } else {
                        String::from("")
                    })
                    .description(if let Some(description) = campaign.description {
                        description
                    } else {
                        String::from("")
                    })
                    .field("DM", format!("<@{}>", campaign.dm_id), false),
            )
        });
    }

    responses::paginate_embeds(ctx, embeds).await
}
