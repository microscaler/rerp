-- Migration: bulk_jobs
-- Generated: 20261110131017

CREATE TABLE IF NOT EXISTS audience.bulk_jobs (
    workspace_id UUID NOT NULL,
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    label TEXT,
    action TEXT NOT NULL,
    value TEXT,
    field_key TEXT,
    list_id UUID REFERENCES audience.lists(id) ON DELETE SET NULL,
    segment_id UUID REFERENCES audience.segments(id) ON DELETE SET NULL,
    contact_total BIGINT,
    status TEXT NOT NULL,
    started_by TEXT,
    undoable BOOLEAN,
    undo_until TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    finished_at TIMESTAMPTZ,
    undone_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

ALTER TABLE audience.bulk_jobs DROP CONSTRAINT IF EXISTS check_bulk_jobs_action;
ALTER TABLE audience.bulk_jobs ADD CONSTRAINT check_bulk_jobs_action CHECK (action IN ('ADD_TAG', 'REMOVE_TAG', 'SUBSCRIBE', 'UNSUBSCRIBE', 'SET_FIELD', 'ARCHIVE'));
ALTER TABLE audience.bulk_jobs DROP CONSTRAINT IF EXISTS check_bulk_jobs_status;
ALTER TABLE audience.bulk_jobs ADD CONSTRAINT check_bulk_jobs_status CHECK (status IN ('QUEUED', 'RUNNING', 'DONE', 'FAILED', 'UNDONE'));
CREATE INDEX IF NOT EXISTS idx_bulk_jobs_list_id ON audience.bulk_jobs(list_id);
CREATE INDEX IF NOT EXISTS idx_bulk_jobs_segment_id ON audience.bulk_jobs(segment_id);
COMMENT ON TABLE audience.bulk_jobs IS 'Changes applied to many contacts; undoable for 7 days';
