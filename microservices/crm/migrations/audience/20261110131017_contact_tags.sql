-- Migration: contact_tags
-- Generated: 20261110131017

CREATE TABLE IF NOT EXISTS audience.contact_tags (
    workspace_id UUID NOT NULL,
    contact_id UUID REFERENCES audience.contacts(id) ON DELETE CASCADE,
    tag_id UUID REFERENCES audience.tags(id) ON DELETE CASCADE,
    source TEXT NOT NULL DEFAULT 'USER',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (contact_id, tag_id)
);

ALTER TABLE audience.contact_tags DROP CONSTRAINT IF EXISTS check_contact_tags_source;
ALTER TABLE audience.contact_tags ADD CONSTRAINT check_contact_tags_source CHECK (source IN ('USER', 'AUTOMATION', 'BULK_JOB', 'IMPORT', 'FORM'));
COMMENT ON TABLE audience.contact_tags IS 'Which contacts carry which tags';
