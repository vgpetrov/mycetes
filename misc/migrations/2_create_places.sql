CREATE TABLE place (
    id bigserial PRIMARY KEY,
    name text NOT NULL,
    user_id bigserial NOT NULL,
    latitude double precision NOT NULL,
    longitude double precision NOT NULL,
    metadata jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted boolean NOT NULL DEFAULT false
);
CREATE INDEX ON place (latitude);
CREATE INDEX ON place (longitude);

-- Add comments
-- COMMENT ON TABLE place IS 'Stores places with coordinates and metadata';
-- COMMENT ON COLUMN place.id IS 'Primary key, auto-incrementing identifier';
-- COMMENT ON COLUMN place.name IS 'Human-readable name of the place';
-- COMMENT ON COLUMN place.user_id IS 'Owner user reference';
-- COMMENT ON COLUMN place.latitude IS 'Latitude in WGS84 coordinate system';
-- COMMENT ON COLUMN place.longitude IS 'Longitude in WGS84 coordinate system';
-- COMMENT ON COLUMN place.deleted IS 'Soft delete flag';