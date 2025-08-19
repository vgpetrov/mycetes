CREATE TABLE places (
    id bigserial PRIMARY KEY,
    name text NOT NULL,
    user_id bigserial NOT NULL,
    latitude double precision NOT NULL,
    longitude double precision NOT NULL,
    is_deleted boolean NOT NULL DEFAULT false
);
CREATE INDEX ON places (latitude);
CREATE INDEX ON places (longitude);