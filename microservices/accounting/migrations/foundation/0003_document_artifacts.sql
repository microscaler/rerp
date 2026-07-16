-- Immutable metadata for rendered Accounting documents held in private,
-- content-addressed object storage. Object bytes never live in PostgreSQL.

-- The tenant policy is the completion marker for this pre-ledger migration.
-- A fresh application is transactional; a completed application is skipped.
SELECT EXISTS (
    SELECT 1
    FROM pg_policies
    WHERE schemaname = current_schema()
      AND tablename = 'accounting_document_artifacts'
      AND policyname = 'accounting_document_artifacts_tenant'
) AS accounting_document_artifacts_installed \gset

\if :accounting_document_artifacts_installed
\echo 'Accounting document artifacts already installed; skipping 0003.'
\else
BEGIN;

CREATE TABLE accounting_document_artifacts (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR(200) NOT NULL,
    legal_entity_id UUID NOT NULL,
    document_id UUID NOT NULL,
    media_type VARCHAR(100) NOT NULL,
    storage_provider VARCHAR(100) NOT NULL,
    bucket VARCHAR(255) NOT NULL,
    object_key VARCHAR(1024) NOT NULL,
    sha256 VARCHAR(64) NOT NULL,
    size_bytes BIGINT NOT NULL,
    renderer VARCHAR(30) NOT NULL,
    renderer_version VARCHAR(30) NOT NULL,
    rendered_at TIMESTAMP NOT NULL,
    rendered_by UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT accounting_document_artifacts_document_format_unique
        UNIQUE (tenant_id, legal_entity_id, document_id, media_type),
    CONSTRAINT accounting_document_artifacts_scope_id_unique
        UNIQUE (tenant_id, legal_entity_id, id),
    CONSTRAINT accounting_document_artifacts_document_scope_fk
        FOREIGN KEY (tenant_id, legal_entity_id, document_id)
        REFERENCES accounting_posted_documents(tenant_id, legal_entity_id, id)
        ON DELETE RESTRICT,
    CONSTRAINT accounting_document_artifacts_provider_check
        CHECK (storage_provider = 'S3_COMPATIBLE'),
    CONSTRAINT accounting_document_artifacts_media_type_check
        CHECK (media_type = 'application/pdf'),
    CONSTRAINT accounting_document_artifacts_checksum_check
        CHECK (sha256 ~ '^[0-9a-f]{64}$'),
    CONSTRAINT accounting_document_artifacts_size_check
        CHECK (size_bytes > 0),
    CONSTRAINT accounting_document_artifacts_key_check
        CHECK (object_key !~ '(^|/)\.\.(/|$)' AND object_key !~ '^/')
);

CREATE INDEX idx_accounting_document_artifacts_scope
    ON accounting_document_artifacts(tenant_id, legal_entity_id);
CREATE INDEX idx_accounting_document_artifacts_document
    ON accounting_document_artifacts(document_id);
CREATE INDEX idx_accounting_document_artifacts_checksum
    ON accounting_document_artifacts(sha256);

CREATE TRIGGER accounting_document_artifacts_immutable
BEFORE UPDATE OR DELETE ON accounting_document_artifacts
FOR EACH ROW EXECUTE FUNCTION public.accounting_reject_immutable_mutation();

ALTER TABLE accounting_document_artifacts ENABLE ROW LEVEL SECURITY;
ALTER TABLE accounting_document_artifacts FORCE ROW LEVEL SECURITY;
CREATE POLICY accounting_document_artifacts_tenant ON accounting_document_artifacts
FOR ALL TO PUBLIC
USING (tenant_id = public.sesame_current_tenant_id())
WITH CHECK (tenant_id = public.sesame_current_tenant_id());

COMMIT;
\endif
