-- Migration: subscriptions
-- Generated: 20261110131017

CREATE TABLE IF NOT EXISTS audience.subscriptions (
    workspace_id UUID NOT NULL,
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    contact_id UUID NOT NULL REFERENCES audience.contacts(id) ON DELETE CASCADE,
    list_id UUID NOT NULL REFERENCES audience.lists(id) ON DELETE CASCADE,
    status TEXT NOT NULL,
    consent_source TEXT,
    consent_at TIMESTAMPTZ,
    consent_ip VARCHAR(45),
    consent_text TEXT,
    form_id UUID,
    unsubscribed_at TIMESTAMPTZ,
    unsubscribe_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    confirmed_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_subscriptions_list_id_contact_id ON audience.subscriptions(list_id, contact_id);
CREATE INDEX IF NOT EXISTS idx_subscriptions_status ON audience.subscriptions(status);
ALTER TABLE audience.subscriptions DROP CONSTRAINT IF EXISTS check_subscriptions_status;
ALTER TABLE audience.subscriptions ADD CONSTRAINT check_subscriptions_status CHECK (status IN ('PENDING', 'SUBSCRIBED', 'UNSUBSCRIBED', 'BOUNCED', 'COMPLAINED'));
ALTER TABLE audience.subscriptions DROP CONSTRAINT IF EXISTS check_subscriptions_consent_source;
ALTER TABLE audience.subscriptions ADD CONSTRAINT check_subscriptions_consent_source CHECK (consent_source IN ('WEB_FORM', 'DOUBLE_OPT_IN', 'IMPORT', 'STAFF', 'API', 'PREFERENCE_PAGE', 'CHECKOUT'));
CREATE INDEX IF NOT EXISTS idx_subscriptions_contact_id ON audience.subscriptions(contact_id);
CREATE INDEX IF NOT EXISTS idx_subscriptions_list_id ON audience.subscriptions(list_id);
COMMENT ON TABLE audience.subscriptions IS 'One contact on one list, with the consent that put them there';
