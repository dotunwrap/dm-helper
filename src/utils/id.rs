use poise::serenity_prelude as serenity;

pub async fn user_id_to_i64(id: serenity::model::id::UserId) -> i64 {
    id_to_i64(id.get()).await
}

pub async fn guild_id_to_i64(id: serenity::model::id::GuildId) -> i64 {
    id_to_i64(id.get()).await
}

pub async fn channel_id_to_i64(id: serenity::model::id::ChannelId) -> i64 {
    id_to_i64(id.get()).await
}

pub async fn role_id_to_i64(id: serenity::model::id::RoleId) -> i64 {
    id_to_i64(id.get()).await
}

pub async fn i64_to_user_id(id: i64) -> serenity::model::id::UserId {
    serenity::model::id::UserId::from(i64_to_u64(id).await)
}

pub async fn i64_to_guild_id(id: i64) -> serenity::model::id::GuildId {
    serenity::model::id::GuildId::from(i64_to_u64(id).await)
}

pub async fn i64_to_channel_id(id: i64) -> serenity::model::id::ChannelId {
    serenity::model::id::ChannelId::from(i64_to_u64(id).await)
}

pub async fn i64_to_role_id(id: i64) -> serenity::model::id::RoleId {
    serenity::model::id::RoleId::from(i64_to_u64(id).await)
}

async fn id_to_i64(id: u64) -> i64 {
    i64::try_from(id).expect("Failed to convert ID to i64")
}

async fn i64_to_u64(id: i64) -> u64 {
    u64::try_from(id).expect("Failed to convert ID to u64")
}
