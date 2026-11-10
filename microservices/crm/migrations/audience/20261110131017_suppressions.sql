-- Migration: suppressions
-- Generated: 20261110131017

CREATE TABLE IF NOT EXISTS audience.suppressions (
    workspace_id UUID NOT NULL,
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    address TEXT NOT NULL,
    channel TEXT NOT NULL DEFAULT 'EMAIL',
    reason TEXT NOT NULL,
    note TEXT,
    source_send_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    source TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_suppressions_workspace_id_channel_address ON audience.suppressions(workspace_id, channel, lower(address));
ALTER TABLE audience.suppressions DROP CONSTRAINT IF EXISTS check_suppressions_channel;
ALTER TABLE audience.suppressions ADD CONSTRAINT check_suppressions_channel CHECK (channel IN ('EMAIL', 'SMS'));
ALTER TABLE audience.suppressions DROP CONSTRAINT IF EXISTS check_suppressions_reason;
ALTER TABLE audience.suppressions ADD CONSTRAINT check_suppressions_reason CHECK (reason IN ('HARD_BOUNCE', 'COMPLAINT', 'MANUAL', 'GLOBAL_UNSUBSCRIBE'));
COMMENT ON TABLE audience.suppressions IS 'Addresses no list may message (hard bounce, complaint, manual, erased)';
