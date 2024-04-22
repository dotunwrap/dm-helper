# <p align="center"><img src="assets/banner.png" style="width: 450; margin: 1rem 0 0 0" /></p>

A WIP Discord bot written in Rust using Serenity and Poise to moderate and manage D&amp;D campaigns.
This bot runs on [Shuttle.rs](https://shuttle.rs). All PRs to master will deploy automatically to the production environment using GitHub Actions.
This project is not affiliated with Discord, Wizards of the Coast, or Dungeons &amp; Dragons.

## Adding this bot to your server

If you'd like to add this bot to your server, you can [CLICK HERE](https://discord.com/oauth2/authorize?client_id=1201638205389275216&permissions=274877992000&scope=applications.commands+bot). This link will automatically request basic bot permissions. These include:

- View channels
- Read messages
- Send messages
- Send messages in threads
- Embed links
- Read message history
- Add reactions

## Development

In order to work on this project, you will need to run the following commands:

```bash
git clone git@github.com:dotunwrap/dm-helper.git
cd dm-helper
```

You will then need to copy the `Secrets.dev.toml.template` file to `Secrets.dev.toml` by running:

```bash
cp Secrets.dev.toml.template Secrets.dev.toml
```

You can then fill out the values in the `Secrets.dev.toml` file.
`DISCORD_TOKEN` should be the token for your bot. If you do not have a bot yet, you can create one [here](https://discord.com/developers/applications).
`DATABASE_URL` should be a URL to a Postgres database.

To make development easy, this project utilizes automated schema migrations. To bootstrap your database to match the correct schema, you'll need a few things to get started.
As this project utilized the [Diesel ORM](https://github.com/diesel-rs/diesel), you'll need to install the Diesel CLI. You can do so by running:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

After you have installed the CLI, you can run the following from the root of the project:

HINT: The format for <DATABASE_URL> is:
```bash
postgres://username:password@localhost:5432/database_name
```

```bash
diesel migration run --database-url <DATABASE_URL>
```

You can then run:

```bash
cargo shuttle run
```

Your bot should now be running on your machine. Add the bot to your server, and you can test locally.

### PR Guide

When submitting a PR, please base it on the `develop` branch.
Please give a detailed description of the features added, bugs fixed, or any other important information regarding the changes made.
The `develop` branch will be merged into `master` when the next release is released, and all changes staged in the branch from merged PRs will deploy then.
