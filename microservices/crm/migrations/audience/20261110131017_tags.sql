-- Migration: tags
-- Generated: 20261110131017

CREATE TABLE IF NOT EXISTS audience.tags (
    workspace_id UUID NOT NULL,
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(64) NOT NULL,
    color CHAR(7),
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_tags_workspace_id_name ON audience.tags(workspace_id, lower(name));
COMMENT ON TABLE audience.tags IS 'Tag definitions; names unique per workspace';
