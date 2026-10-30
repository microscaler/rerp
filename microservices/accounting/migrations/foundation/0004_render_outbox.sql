-- Durable render-outbox instructions for posted Accounting documents
-- (NFR-DOG-010): stored atomically with the posting together with the
-- immutable render model, so rendering is durable, retryable and
-- operator-inspectable while the render pipeline or object storage is down.

-- The tenant policy is the completion marker for this migration.
-- A fresh application is transactional; a completed application is skipped.
SELECT EXISTS (
    SELECT 1
    FROM pg_policies
    WHERE schemaname = current_schema()
      AND tablename = 'accounting_render_outbox'
      AND policyname = 'accounting_render_outbox_tenant'
) AS accounting_render_outbox_installed \gset

\if :accounting_render_outbox_installed
\echo 'Accounting render outbox already installed; skipping 0004.'
\else
BEGIN;

CREATE TABLE accounting_render_outbox (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR(200) NOT NULL,
    legal_entity_id UUID NOT NULL,
    document_id UUID NOT NULL,
    media_type VARCHAR(100) NOT NULL,
    renderer VARCHAR(30) NOT NULL,
    renderer_version VARCHAR(30) NOT NULL,
    render_model JSONB NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'PENDING',
    attempts INTEGER NOT NULL DEFAULT 0,
    last_error VARCHAR(500),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL,
    CONSTRAINT accounting_render_outbox_document_format_unique
        UNIQUE (tenant_id, legal_entity_id, document_id, media_type),
    CONSTRAINT accounting_render_outbox_scope_id_unique
        UNIQUE (tenant_id, legal_entity_id, id),
    CONSTRAINT accounting_render_outbox_document_scope_fk
        FOREIGN KEY (tenant_id, legal_entity_id, document_id)
        REFERENCES accounting_posted_documents(tenant_id, legal_entity_id, id)
        ON DELETE RESTRICT,
    CONSTRAINT accounting_render_outbox_status_check
        CHECK (status IN ('PENDING', 'IN_PROGRESS', 'DONE')),
    CONSTRAINT accounting_render_outbox_media_type_check
        CHECK (media_type = 'application/pdf'),
    CONSTRAINT accounting_render_outbox_attempts_check
        CHECK (attempts >= 0)
);

CREATE INDEX idx_accounting_render_outbox_scope
    ON accounting_render_outbox(tenant_id, legal_entity_id);
CREATE INDEX idx_accounting_render_outbox_status
    ON accounting_render_outbox(status);
CREATE INDEX idx_accounting_render_outbox_document
    ON accounting_render_outbox(document_id);

ALTER TABLE accounting_render_outbox ENABLE ROW LEVEL SECURITY;
ALTER TABLE accounting_render_outbox FORCE ROW LEVEL SECURITY;
CREATE POLICY accounting_render_outbox_tenant ON accounting_render_outbox
FOR ALL TO PUBLIC
USING (tenant_id = public.sesame_current_tenant_id())
WITH CHECK (tenant_id = public.sesame_current_tenant_id());

COMMIT;
\endif
