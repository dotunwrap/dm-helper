CREATE TABLE characters (
    id serial NOT NULL,
    campaign_id integer NOT NULL,
    player_id bigint NOT NULL,
    name text NOT NULL,
    race text NOT NULL,
    class text NOT NULL,
    CONSTRAINT pk_characters PRIMARY KEY (id),
    CONSTRAINT fk_characters_campaigns FOREIGN KEY (campaign_id) REFERENCES campaigns (id),
    CONSTRAINT uk_campaign_id_player_id UNIQUE (campaign_id, player_id)
);

CREATE INDEX idx_characters_campaign_id ON characters (campaign_id);
CREATE INDEX idx_characters_player_id ON characters (player_id);
