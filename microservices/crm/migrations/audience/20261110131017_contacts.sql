-- Migration: contacts
-- Generated: 20261110131017

CREATE TABLE IF NOT EXISTS audience.contacts (
    workspace_id UUID NOT NULL,
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL,
    first_name VARCHAR(128),
    last_name VARCHAR(128),
    phone VARCHAR(32),
    job_title VARCHAR(128),
    organisation_id UUID REFERENCES audience.organisations(id) ON DELETE SET NULL,
    country_code CHAR(2),
    timezone TEXT,
    custom_fields JSONB NOT NULL DEFAULT '{}'::jsonb,
    source TEXT,
    attribution JSONB,
    sms_consent BOOLEAN NOT NULL DEFAULT false,
    tracking_optout BOOLEAN NOT NULL DEFAULT false,
    last_activity_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    status TEXT NOT NULL DEFAULT 'PENDING',
    sms_consent_at TIMESTAMPTZ,
    archived_at TIMESTAMPTZ,
    erased_at TIMESTAMPTZ,
    created_by VARCHAR(64)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_contacts_workspace_id_email ON audience.contacts(workspace_id, lower(email));
CREATE INDEX IF NOT EXISTS idx_contacts_status ON audience.contacts(status);
CREATE INDEX IF NOT EXISTS idx_contacts_last_activity_at ON audience.contacts(last_activity_at);
CREATE INDEX IF NOT EXISTS idx_contacts_created_at ON audience.contacts(created_at);
ALTER TABLE audience.contacts DROP CONSTRAINT IF EXISTS check_contacts_status;
ALTER TABLE audience.contacts ADD CONSTRAINT check_contacts_status CHECK (status IN ('SUBSCRIBED', 'UNSUBSCRIBED', 'PENDING', 'BOUNCED'));
CREATE INDEX IF NOT EXISTS idx_contacts_organisation_id ON audience.contacts(organisation_id);
COMMENT ON TABLE audience.contacts IS 'People the workspace can reach. Email unique per workspace (case-insensitive)';
