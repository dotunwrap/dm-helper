# dm-helper
A WIP Discord bot written in Rust using Serenity and Poise to moderate and manage D&amp;D campaigns.
This bot runs on [Shuttle.rs](https://shuttle.rs). All PRs to master will deploy automatically to the production environment using GitHub Actions.

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
`DATABASE_URL` should be a URL to a MySQL database. The database should be have the following tables:
```sql
CREATE TABLE `campaigns` (
    `id` int unsigned NOT NULL AUTO_INCREMENT,
    `guild_id` varchar(255) NOT NULL,
    `dm_id` varchar(255) DEFAULT NULL,
    `name` text,
    `description` text,
    PRIMARY KEY (`id`)
);

CREATE TABLE `characters` (
    `id` int NOT NULL AUTO_INCREMENT,
    `campaign_id` int DEFAULT NULL,
    `player_id` varchar(255) DEFAULT NULL,
    `name` text,
    `race` text,
    `class` text,
    PRIMARY KEY (`id`),
    UNIQUE KEY `unique_campaign_player` (`campaign_id`,`player_id`)
    KEY `player_id_index` (`player_id`)
    KEY `campaign_id_index` (`campaign_id`)
);

CREATE TABLE `sessions` (
    `id` int unsigned NOT NULL AUTO_INCREMENT,
    `campaign_id` int DEFAULT NULL,
    `author_id` varchar(255) DEFAULT NULL,
    `location` text,
    `status` int DEFAULT NULL,
    `created_date` datetime NOT NULL,
    `scheduled_date` datetime DEFAULT NULL,
    PRIMARY KEY (`id`),
    KEY `campaign_id_index` (`campaign_id`)
);

CREATE TABLE `responses` (
    `id` int unsigned NOT NULL AUTO_INCREMENT,
    `session_id` int DEFAULT NULL,
    `respondee_id` varchar(255) DEFAULT NULL,
    `response` int DEFAULT NULL,
    `responded_date` datetime DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `unique_session_respondee` (`session_id`,`respondee_id`),
    KEY `session_id_index` (`session_id`)
);

CREATE TABLE `settings` (
    `guild_id` varchar(255) NOT NULL,
    `dnd_role` varchar(255) DEFAULT NULL,
    `dm_role` varchar(255) DEFAULT NULL,
    PRIMARY KEY (`guild_id`)
);
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
