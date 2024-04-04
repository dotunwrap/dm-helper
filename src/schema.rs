// @generated automatically by Diesel CLI.

diesel::table! {
    campaigns (id) {
        id -> Int4,
        guild_id -> Int8,
        dm_id -> Int8,
        name -> Text,
        description -> Nullable<Text>,
        link -> Nullable<Text>,
        deleted -> Bool,
        created_date -> Timestamp,
    }
}

diesel::table! {
    characters (id) {
        id -> Int4,
        campaign_id -> Int4,
        player_id -> Int8,
        name -> Text,
        race -> Text,
        class -> Text,
    }
}

diesel::table! {
    responses (id) {
        id -> Int4,
        session_id -> Int4,
        respondee_id -> Int8,
        response -> Int2,
        responded_date -> Timestamp,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        campaign_id -> Int4,
        author_id -> Int8,
        location -> Nullable<Text>,
        status -> Int2,
        created_date -> Timestamp,
        scheduled_date -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    settings (guild_id) {
        guild_id -> Int8,
        dnd_role_id -> Nullable<Int8>,
        dm_role_id -> Nullable<Int8>,
    }
}

diesel::joinable!(characters -> campaigns (campaign_id));
diesel::joinable!(responses -> sessions (session_id));
diesel::joinable!(sessions -> campaigns (campaign_id));

diesel::allow_tables_to_appear_in_same_query!(
    campaigns,
    characters,
    responses,
    sessions,
    settings,
);
