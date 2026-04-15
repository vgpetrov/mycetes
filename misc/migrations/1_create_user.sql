CREATE TABLE "user" (
    id bigserial PRIMARY KEY,
    name text NOT NULL,
    email text NOT NULL,
    password text NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted boolean NOT NULL DEFAULT false
);

CREATE INDEX ON "user" (email);