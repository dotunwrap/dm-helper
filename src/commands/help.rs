use crate::{Context, Error};

/// Shows the help menu
///
/// If no command is provided, shows the help menu
/// If a command is provided, shows the help menu for that command
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to get help with"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "\
            Type .help <command> for more info on a command.",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}
