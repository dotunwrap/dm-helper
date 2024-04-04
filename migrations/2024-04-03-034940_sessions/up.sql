CREATE TABLE sessions (
    id serial NOT NULL,
    campaign_id integer NOT NULL,
    author_id bigint NOT NULL,
    location text,
    status smallint NOT NULL DEFAULT 0,
    created_date timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    scheduled_date timestamp with time zone DEFAULT NULL,
    CONSTRAINT pk_sessions PRIMARY KEY (id),
    CONSTRAINT fk_sessions_campaigns FOREIGN KEY (campaign_id) REFERENCES campaigns (id)
);

CREATE INDEX idx_sessions_campaign_id ON sessions (campaign_id);
