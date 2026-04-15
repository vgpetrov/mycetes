CREATE TABLE spot (
    id bigserial PRIMARY KEY,
    name text NOT NULL,
    user_id bigserial NOT NULL,
    latitude double precision NOT NULL,
    longitude double precision NOT NULL,
    metadata jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted boolean NOT NULL DEFAULT false
);
CREATE INDEX ON spot (latitude);
CREATE INDEX ON spot (longitude);