-- Migration: organisations
-- Generated: 20261110131017

CREATE TABLE IF NOT EXISTS audience.organisations (
    workspace_id UUID NOT NULL,
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    domain VARCHAR(255),
    website TEXT,
    industry VARCHAR(128),
    size TEXT,
    country_code CHAR(2),
    parent_id UUID REFERENCES audience.organisations(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    customer_account_id UUID,
    converted_at TIMESTAMPTZ,
    archived_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_organisations_workspace_id_domain ON audience.organisations(workspace_id, lower(domain));
CREATE INDEX IF NOT EXISTS idx_organisations_name ON audience.organisations(name);
ALTER TABLE audience.organisations DROP CONSTRAINT IF EXISTS check_organisations_size;
ALTER TABLE audience.organisations ADD CONSTRAINT check_organisations_size CHECK (size IN ('1-10', '11-50', '51-200', '201-1000', '1001-5000', '5000+'));
CREATE INDEX IF NOT EXISTS idx_organisations_parent_id ON audience.organisations(parent_id);
COMMENT ON TABLE audience.organisations IS 'Prospect companies. A customer exists only in RERP crm/accounts (ADR 003)';
