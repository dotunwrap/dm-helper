#![warn(clippy::str_to_string)]

use crate::commands::*;
use dotenv::dotenv;
use poise::serenity_prelude as serenity;

pub mod commands;
pub mod responses;
pub mod utils;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    db: mysql::Pool,
    dnd_role: serenity::RoleId,
}

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

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(on_event(_ctx, event, _framework, _data))
            },
            commands: vec![
                help::help(),
                settings::settings(),
                dnd::campaign::session::session(),
                dnd::dice::roll(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                edit_tracker: Some(Into::into(poise::EditTracker::for_timespan(
                    std::time::Duration::from_secs(60),
                ))),
                ..Default::default()
            },
            on_error: |error| Box::pin(on_error(error)),
            command_check: Some(|ctx| {
                Box::pin(async move {
                    Ok(ctx
                        .author_member()
                        .await
                        .unwrap()
                        .roles
                        .contains(&serenity::RoleId::from(ctx.data().dnd_role)))
                })
            }),
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    db: utils::db::init_dnd_db(),
                    dnd_role: serenity::RoleId::from(901464574530814002),
                })
            })
        })
        .build();

    serenity::ClientBuilder::new(
        std::env::var("DISCORD_TOKEN").expect("Missing Discord token"),
        serenity::GatewayIntents::non_privileged()
            | serenity::GatewayIntents::MESSAGE_CONTENT
            | serenity::GatewayIntents::GUILD_MEMBERS,
    )
    .framework(framework)
    .await
    .unwrap()
    .start()
    .await
    .unwrap();
}
