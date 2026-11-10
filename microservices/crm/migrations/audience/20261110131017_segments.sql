-- Migration: segments
-- Generated: 20261110131017

CREATE TABLE IF NOT EXISTS audience.segments (
    workspace_id UUID NOT NULL,
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(128) NOT NULL,
    description TEXT,
    conditions JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    contact_count BIGINT,
    counted_at TIMESTAMPTZ
);

COMMENT ON TABLE audience.segments IS 'Saved segments: a two-level condition tree over the contact facts';
