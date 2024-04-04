CREATE TABLE settings (
    guild_id bigint NOT NULL,
    dnd_role_id bigint DEFAULT NULL,
    dm_role_id bigint DEFAULT NULL,
    CONSTRAINT pk_settings PRIMARY KEY (guild_id)
);

CREATE INDEX idx_settings_guild_id ON settings (guild_id);
