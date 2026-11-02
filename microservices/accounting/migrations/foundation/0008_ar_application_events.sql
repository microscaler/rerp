-- Payment-application audit journal (Epic F2b): the ar_payment_applications
-- pair row (UNIQUE per payment+invoice) stores CURRENT state, so repeated
-- applications and (F2c) reversals need an immutable journal for the audit
-- linkage the epic demands.

-- The tenant policy is the completion marker for this migration.
-- A fresh application is transactional; a completed application is skipped.
SELECT EXISTS (
    SELECT 1
    FROM pg_policies
    WHERE schemaname = current_schema()
      AND tablename = 'ar_payment_application_events'
      AND policyname = 'ar_payment_application_events_tenant'
) AS ar_application_events_installed \gset

\if :ar_application_events_installed
\echo 'AR application events journal already installed; skipping 0008.'
\else
BEGIN;

CREATE TABLE ar_payment_application_events (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR(200) NOT NULL,
    legal_entity_id UUID NOT NULL,
    application_id UUID NOT NULL,
    kind VARCHAR(20) NOT NULL,
    amount NUMERIC(19, 6) NOT NULL CHECK (amount > 0),
    actor_id UUID,
    occurred_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    reason VARCHAR(1000),
    CONSTRAINT ar_payment_application_events_scope_id_unique
        UNIQUE (tenant_id, legal_entity_id, id),
    CONSTRAINT ar_payment_application_events_kind_check
        CHECK (kind IN ('APPLIED', 'REVERSED')),
    CONSTRAINT ar_payment_application_events_application_scope_fk
        FOREIGN KEY (tenant_id, legal_entity_id, application_id)
        REFERENCES ar_payment_applications(tenant_id, legal_entity_id, id)
        ON DELETE CASCADE
);

CREATE INDEX idx_ar_payment_application_events_scope
    ON ar_payment_application_events(tenant_id, legal_entity_id);
CREATE INDEX idx_ar_payment_application_events_application
    ON ar_payment_application_events(application_id);

-- The journal is append-only: the runtime role must not be able to edit
-- history (same discipline as the other accounting journals).
CREATE TRIGGER ar_payment_application_events_immutable
BEFORE UPDATE OR DELETE ON ar_payment_application_events
FOR EACH ROW EXECUTE FUNCTION public.accounting_reject_immutable_mutation();

ALTER TABLE ar_payment_application_events ENABLE ROW LEVEL SECURITY;
ALTER TABLE ar_payment_application_events FORCE ROW LEVEL SECURITY;
CREATE POLICY ar_payment_application_events_tenant ON ar_payment_application_events
FOR ALL TO PUBLIC
USING (tenant_id = public.sesame_current_tenant_id())
WITH CHECK (tenant_id = public.sesame_current_tenant_id());

COMMIT;
\endif
