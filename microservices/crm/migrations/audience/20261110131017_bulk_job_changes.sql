-- Migration: bulk_job_changes
-- Generated: 20261110131017

CREATE TABLE IF NOT EXISTS audience.bulk_job_changes (
    workspace_id UUID NOT NULL,
    job_id UUID REFERENCES audience.bulk_jobs(id) ON DELETE CASCADE,
    contact_id UUID,
    before JSONB NOT NULL,
    after JSONB NOT NULL,
    PRIMARY KEY (job_id, contact_id)
);

COMMENT ON TABLE audience.bulk_job_changes IS 'Before and after values per contact, for undo';
