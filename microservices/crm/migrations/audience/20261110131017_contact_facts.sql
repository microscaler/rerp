-- Migration: contact_facts
-- Generated: 20261110131017

CREATE TABLE IF NOT EXISTS audience.contact_facts (
    workspace_id UUID NOT NULL,
    contact_id UUID PRIMARY KEY DEFAULT gen_random_uuid() REFERENCES audience.contacts(id) ON DELETE CASCADE,
    last_opened_at TIMESTAMPTZ,
    last_clicked_at TIMESTAMPTZ,
    last_sent_at TIMESTAMPTZ,
    opens_30d INTEGER NOT NULL DEFAULT '0',
    clicks_30d INTEGER NOT NULL DEFAULT '0',
    pages_visited_30d JSONB,
    events_30d JSONB,
    forms_submitted JSONB,
    deal_stage_ids JSONB,
    open_deal_count INTEGER NOT NULL DEFAULT '0',
    score_bands JSONB,
    active_workflow_ids JSONB,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

COMMENT ON TABLE audience.contact_facts IS 'Per-contact projection of facts from other services, so a segment is one query';
