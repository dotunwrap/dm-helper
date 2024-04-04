CREATE TABLE responses (
    id serial NOT NULL,
    session_id integer NOT NULL,
    respondee_id bigint NOT NULL,
    response smallint NOT NULL DEFAULT 0,
    responded_date timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_responses PRIMARY KEY (id),
    CONSTRAINT fk_responses_session FOREIGN KEY (session_id) REFERENCES sessions(id),
    CONSTRAINT uk_session_id_respondee_id UNIQUE (session_id, respondee_id)
);

CREATE INDEX idx_responses_session_id ON responses (session_id);
