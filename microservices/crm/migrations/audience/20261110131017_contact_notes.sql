-- Migration: contact_notes
-- Generated: 20261110131017

CREATE TABLE IF NOT EXISTS audience.contact_notes (
    workspace_id UUID NOT NULL,
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    contact_id UUID NOT NULL REFERENCES audience.contacts(id) ON DELETE CASCADE,
    body TEXT NOT NULL,
    pinned BOOLEAN NOT NULL DEFAULT false,
    created_by VARCHAR(64),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_contact_notes_contact_id_created_at ON audience.contact_notes(contact_id, created_at);
CREATE INDEX IF NOT EXISTS idx_contact_notes_contact_id ON audience.contact_notes(contact_id);
COMMENT ON TABLE audience.contact_notes IS 'Notes on a contact';
