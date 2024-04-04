#![warn(clippy::str_to_string)]
#[macro_use]
extern crate diesel;

use crate::commands::*;
use anyhow::anyhow;
use poise::serenity_prelude as serenity;
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;

pub mod commands;
pub mod models;
pub mod ops;
pub mod responses;
pub mod schema;
pub mod utils;

pub struct Data {
    db_pool: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) -> () {
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            panic!("Failed to build framework: {:?}", error)
        }
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!(
                "Error in command `{}`: {:?}",
                ctx.command().qualified_name,
                error
            );

            responses::failure(ctx, "Something went wrong.")
                .await
                .unwrap_or_default();
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Failed to call on_error: {:?}", e);
            }
        }
    }
}

async fn on_event(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot } => {
            println!("{} is connected!", data_about_bot.user.name);
        }
        _ => {}
    }

    Ok(())
}

#[shuttle_runtime::main]
pub async fn poise(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("DISCORD_TOKEN not found in secret store").into());
    };

    let database_url = if let Some(url) = secret_store.get("DATABASE_URL") {
        url
    } else {
        return Err(anyhow!("DATABASE_URL not found in secret store").into());
    };

    let commands = vec![
        help::help(),
        settings::settings(),
        dnd::campaign::session::session(),
        dnd::campaign::campaign(),
        dnd::dice::roll(),
    ];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(on_event(_ctx, event, _framework, _data))
            },
            commands,
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                edit_tracker: Some(Into::into(poise::EditTracker::for_timespan(
                    std::time::Duration::from_secs(60),
                ))),
                ..Default::default()
            },
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    db_pool: utils::db::init_pool(&database_url),
                })
            })
        })
        .build();

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS;

    let client = serenity::ClientBuilder::new(&token, intents)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
