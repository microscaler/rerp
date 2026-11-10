-- Migration: import_jobs
-- Generated: 20261110131017

CREATE TABLE IF NOT EXISTS audience.import_jobs (
    workspace_id UUID NOT NULL,
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    kind TEXT NOT NULL,
    status TEXT NOT NULL,
    total_rows BIGINT,
    processed_rows BIGINT,
    created_rows BIGINT,
    updated_rows BIGINT,
    skipped_rows BIGINT,
    errors JSONB,
    download_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    finished_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

ALTER TABLE audience.import_jobs DROP CONSTRAINT IF EXISTS check_import_jobs_kind;
ALTER TABLE audience.import_jobs ADD CONSTRAINT check_import_jobs_kind CHECK (kind IN ('IMPORT', 'EXPORT'));
ALTER TABLE audience.import_jobs DROP CONSTRAINT IF EXISTS check_import_jobs_status;
ALTER TABLE audience.import_jobs ADD CONSTRAINT check_import_jobs_status CHECK (status IN ('QUEUED', 'RUNNING', 'DONE', 'FAILED'));
COMMENT ON TABLE audience.import_jobs IS 'Contact imports and exports run by audience';
