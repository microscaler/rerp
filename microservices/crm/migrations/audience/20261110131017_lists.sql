-- Migration: lists
-- Generated: 20261110131017

CREATE TABLE IF NOT EXISTS audience.lists (
    workspace_id UUID NOT NULL,
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(128) NOT NULL,
    description TEXT,
    sender_id UUID NOT NULL,
    double_opt_in BOOLEAN NOT NULL DEFAULT false,
    confirmation_template_id UUID,
    is_archived BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    channel TEXT NOT NULL DEFAULT 'EMAIL',
    postal_address TEXT,
    reminder_text TEXT
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_lists_workspace_id_name ON audience.lists(workspace_id, lower(name));
ALTER TABLE audience.lists DROP CONSTRAINT IF EXISTS check_lists_channel;
ALTER TABLE audience.lists ADD CONSTRAINT check_lists_channel CHECK (channel IN ('EMAIL', 'SMS'));
COMMENT ON TABLE audience.lists IS 'Mailing lists (email or SMS) with their consent settings';
