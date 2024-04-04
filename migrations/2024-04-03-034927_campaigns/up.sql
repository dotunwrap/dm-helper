CREATE TABLE campaigns (
    id serial NOT NULL,
    guild_id bigint NOT NULL,
    dm_id bigint NOT NULL,
    name text NOT NULL,
    description text,
    link text,
    deleted boolean NOT NULL DEFAULT false, 
    created_date timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_campaigns PRIMARY KEY (id),
    CONSTRAINT uk_guild_id_name UNIQUE (guild_id, name)
);

CREATE INDEX idx_campaigns_guild_id ON campaigns (guild_id);
