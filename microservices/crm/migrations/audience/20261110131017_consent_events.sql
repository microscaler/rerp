-- Migration: consent_events
-- Generated: 20261110131017

CREATE TABLE IF NOT EXISTS audience.consent_events (
    workspace_id UUID NOT NULL,
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    subscription_id UUID NOT NULL REFERENCES audience.subscriptions(id) ON DELETE CASCADE,
    from_status TEXT,
    to_status TEXT NOT NULL,
    source TEXT NOT NULL,
    actor TEXT,
    ip VARCHAR(45),
    note TEXT,
    at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_consent_events_subscription_id_at ON audience.consent_events(subscription_id, at);
ALTER TABLE audience.consent_events DROP CONSTRAINT IF EXISTS check_consent_events_to_status;
ALTER TABLE audience.consent_events ADD CONSTRAINT check_consent_events_to_status CHECK (to_status IN ('PENDING', 'SUBSCRIBED', 'UNSUBSCRIBED', 'BOUNCED'));
CREATE INDEX IF NOT EXISTS idx_consent_events_subscription_id ON audience.consent_events(subscription_id);
COMMENT ON TABLE audience.consent_events IS 'Append-only history of each subscription status change (the consent evidence)';
