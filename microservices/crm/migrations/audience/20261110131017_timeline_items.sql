-- Migration: timeline_items
-- Generated: 20261110131017

CREATE TABLE IF NOT EXISTS audience.timeline_items (
    workspace_id UUID NOT NULL,
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    contact_id UUID NOT NULL REFERENCES audience.contacts(id) ON DELETE CASCADE,
    kind TEXT NOT NULL,
    at TIMESTAMPTZ NOT NULL,
    summary TEXT NOT NULL,
    actor TEXT,
    ref_service TEXT,
    ref_id UUID,
    data JSONB
);

CREATE INDEX IF NOT EXISTS idx_timeline_items_contact_id_at ON audience.timeline_items(contact_id, at DESC);
ALTER TABLE audience.timeline_items DROP CONSTRAINT IF EXISTS check_timeline_items_kind;
ALTER TABLE audience.timeline_items ADD CONSTRAINT check_timeline_items_kind CHECK (kind IN ('MESSAGE_SENT', 'MESSAGE_OPENED', 'MESSAGE_CLICKED', 'MESSAGE_BOUNCED', 'UNSUBSCRIBED', 'SUBSCRIBED', 'FORM_SUBMITTED', 'PAGE_VIEW', 'SITE_EVENT', 'WORKFLOW_ENTERED', 'WORKFLOW_STEP', 'WORKFLOW_ENDED', 'TAG_ADDED', 'TAG_REMOVED', 'FIELD_CHANGED', 'SCORE_CHANGED', 'DEAL_CREATED', 'DEAL_STAGE_CHANGED', 'NOTE', 'CHAT'));
CREATE INDEX IF NOT EXISTS idx_timeline_items_contact_id ON audience.timeline_items(contact_id);
COMMENT ON TABLE audience.timeline_items IS 'The contact timeline, projected from events of every service';
