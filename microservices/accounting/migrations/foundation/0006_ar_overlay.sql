-- Accounts-receivable overlay tables (Epic F1): per-document AR state,
-- payments, allocations and aging snapshots. Truth stays in the immutable
-- posted facts; customer_invoices is a rebuildable overlay anchored to
-- accounting_posted_documents via a tenant-scoped composite FK. Money is
-- NUMERIC(19,6) to match the posted facts (the scaffold's 19,4 was wrong).

-- The tenant policy is the completion marker for this migration.
-- A fresh application is transactional; a completed application is skipped.
SELECT EXISTS (
    SELECT 1
    FROM pg_policies
    WHERE schemaname = current_schema()
      AND tablename = 'customer_invoices'
      AND policyname = 'customer_invoices_tenant'
) AS ar_overlay_installed \gset

\if :ar_overlay_installed
\echo 'AR overlay tables already installed; skipping 0006.'
\else
BEGIN;

CREATE TABLE customer_invoices (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR(200) NOT NULL,
    legal_entity_id UUID NOT NULL,
    invoice_id UUID NOT NULL,
    customer_id UUID NOT NULL,
    original_amount NUMERIC(19, 6) NOT NULL CHECK (original_amount >= 0),
    outstanding_amount NUMERIC(19, 6) NOT NULL DEFAULT 0 CHECK (outstanding_amount >= 0),
    currency_code VARCHAR(3) NOT NULL,
    due_date DATE NOT NULL,
    terms VARCHAR(20) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'CONFIRMED',
    credit_limit_check BOOLEAN,
    write_off_amount NUMERIC(19, 6) CHECK (write_off_amount >= 0),
    write_off_date DATE,
    write_off_reason VARCHAR(1000),
    last_payment_date DATE,
    last_payment_amount NUMERIC(19, 6) CHECK (last_payment_amount >= 0),
    metadata JSONB,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL,
    CONSTRAINT customer_invoices_document_unique
        UNIQUE (tenant_id, legal_entity_id, invoice_id),
    CONSTRAINT customer_invoices_scope_id_unique
        UNIQUE (tenant_id, legal_entity_id, id),
    CONSTRAINT customer_invoices_legal_entity_scope_fk
        FOREIGN KEY (tenant_id, legal_entity_id)
        REFERENCES accounting_legal_entities(tenant_id, id)
        ON DELETE RESTRICT,
    CONSTRAINT customer_invoices_document_scope_fk
        FOREIGN KEY (tenant_id, legal_entity_id, invoice_id)
        REFERENCES accounting_posted_documents(tenant_id, legal_entity_id, id)
        ON DELETE RESTRICT,
    CONSTRAINT customer_invoices_status_check
        CHECK (status IN ('DRAFT', 'CONFIRMED', 'PARTIAL', 'PAID', 'WRITTEN_OFF', 'DISPUTED')),
    CONSTRAINT customer_invoices_terms_check
        CHECK (terms IN ('NET_15', 'NET_30', 'NET_45', 'NET_60', 'DUE_ON_RECEIPT', 'CUSTOM'))
);

CREATE INDEX idx_customer_invoices_scope
    ON customer_invoices(tenant_id, legal_entity_id);
CREATE INDEX idx_customer_invoices_customer
    ON customer_invoices(customer_id);
CREATE INDEX idx_customer_invoices_status
    ON customer_invoices(status);

CREATE TABLE ar_payments (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR(200) NOT NULL,
    legal_entity_id UUID NOT NULL,
    customer_id UUID NOT NULL,
    payment_number VARCHAR(100) NOT NULL,
    payment_date DATE NOT NULL,
    payment_amount NUMERIC(19, 6) NOT NULL DEFAULT 0 CHECK (payment_amount >= 0),
    currency_code VARCHAR(3) NOT NULL DEFAULT 'USD',
    exchange_rate NUMERIC(19, 6) NOT NULL DEFAULT 1.0 CHECK (exchange_rate > 0),
    payment_method VARCHAR(50) NOT NULL,
    payment_reference VARCHAR(100),
    bank_account_id UUID,
    status VARCHAR(50) NOT NULL DEFAULT 'DRAFT',
    reconciled_at TIMESTAMP,
    reconciled_by UUID,
    applied_amount NUMERIC(19, 6) NOT NULL DEFAULT 0 CHECK (applied_amount >= 0),
    unapplied_amount NUMERIC(19, 6) NOT NULL DEFAULT 0 CHECK (unapplied_amount >= 0),
    notes TEXT,
    metadata JSONB,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL,
    created_by UUID,
    updated_by UUID,
    CONSTRAINT ar_payments_number_unique
        UNIQUE (tenant_id, legal_entity_id, payment_number),
    CONSTRAINT ar_payments_scope_id_unique
        UNIQUE (tenant_id, legal_entity_id, id),
    CONSTRAINT ar_payments_legal_entity_scope_fk
        FOREIGN KEY (tenant_id, legal_entity_id)
        REFERENCES accounting_legal_entities(tenant_id, id)
        ON DELETE RESTRICT,
    CONSTRAINT ar_payments_status_check
        CHECK (status IN ('DRAFT', 'POSTED', 'RECONCILED', 'CANCELLED'))
);

CREATE INDEX idx_ar_payments_scope
    ON ar_payments(tenant_id, legal_entity_id);
CREATE INDEX idx_ar_payments_customer
    ON ar_payments(customer_id);
CREATE INDEX idx_ar_payments_payment_date
    ON ar_payments(payment_date);
CREATE INDEX idx_ar_payments_status
    ON ar_payments(status);

CREATE TABLE ar_payment_applications (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR(200) NOT NULL,
    legal_entity_id UUID NOT NULL,
    payment_id UUID NOT NULL,
    invoice_id UUID NOT NULL,
    applied_amount NUMERIC(19, 6) NOT NULL DEFAULT 0 CHECK (applied_amount > 0),
    applied_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    applied_by UUID,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT ar_payment_applications_link_unique
        UNIQUE (tenant_id, legal_entity_id, payment_id, invoice_id),
    CONSTRAINT ar_payment_applications_scope_id_unique
        UNIQUE (tenant_id, legal_entity_id, id),
    CONSTRAINT ar_payment_applications_payment_scope_fk
        FOREIGN KEY (tenant_id, legal_entity_id, payment_id)
        REFERENCES ar_payments(tenant_id, legal_entity_id, id)
        ON DELETE CASCADE,
    CONSTRAINT ar_payment_applications_invoice_scope_fk
        FOREIGN KEY (tenant_id, legal_entity_id, invoice_id)
        REFERENCES customer_invoices(tenant_id, legal_entity_id, id)
        ON DELETE CASCADE
);

CREATE INDEX idx_ar_payment_applications_scope
    ON ar_payment_applications(tenant_id, legal_entity_id);
CREATE INDEX idx_ar_payment_applications_payment
    ON ar_payment_applications(payment_id);
CREATE INDEX idx_ar_payment_applications_invoice
    ON ar_payment_applications(invoice_id);

CREATE TABLE ar_agings (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR(200) NOT NULL,
    legal_entity_id UUID NOT NULL,
    customer_id UUID NOT NULL,
    aging_date DATE NOT NULL,
    current NUMERIC(19, 6) NOT NULL DEFAULT 0 CHECK (current >= 0),
    days_31_60 NUMERIC(19, 6) NOT NULL DEFAULT 0 CHECK (days_31_60 >= 0),
    days_61_90 NUMERIC(19, 6) NOT NULL DEFAULT 0 CHECK (days_61_90 >= 0),
    days_91_120 NUMERIC(19, 6) NOT NULL DEFAULT 0 CHECK (days_91_120 >= 0),
    over_120 NUMERIC(19, 6) NOT NULL DEFAULT 0 CHECK (over_120 >= 0),
    total_outstanding NUMERIC(19, 6) NOT NULL DEFAULT 0 CHECK (total_outstanding >= 0),
    currency_code VARCHAR(3) NOT NULL DEFAULT 'USD',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT ar_agings_asof_unique
        UNIQUE (tenant_id, legal_entity_id, customer_id, aging_date),
    CONSTRAINT ar_agings_scope_id_unique
        UNIQUE (tenant_id, legal_entity_id, id),
    CONSTRAINT ar_agings_legal_entity_scope_fk
        FOREIGN KEY (tenant_id, legal_entity_id)
        REFERENCES accounting_legal_entities(tenant_id, id)
        ON DELETE RESTRICT
);

CREATE INDEX idx_ar_agings_scope
    ON ar_agings(tenant_id, legal_entity_id);
CREATE INDEX idx_ar_agings_customer
    ON ar_agings(customer_id);
CREATE INDEX idx_ar_agings_aging_date
    ON ar_agings(aging_date);

ALTER TABLE customer_invoices ENABLE ROW LEVEL SECURITY;
ALTER TABLE customer_invoices FORCE ROW LEVEL SECURITY;
CREATE POLICY customer_invoices_tenant ON customer_invoices
FOR ALL TO PUBLIC
USING (tenant_id = public.sesame_current_tenant_id())
WITH CHECK (tenant_id = public.sesame_current_tenant_id());

ALTER TABLE ar_payments ENABLE ROW LEVEL SECURITY;
ALTER TABLE ar_payments FORCE ROW LEVEL SECURITY;
CREATE POLICY ar_payments_tenant ON ar_payments
FOR ALL TO PUBLIC
USING (tenant_id = public.sesame_current_tenant_id())
WITH CHECK (tenant_id = public.sesame_current_tenant_id());

ALTER TABLE ar_payment_applications ENABLE ROW LEVEL SECURITY;
ALTER TABLE ar_payment_applications FORCE ROW LEVEL SECURITY;
CREATE POLICY ar_payment_applications_tenant ON ar_payment_applications
FOR ALL TO PUBLIC
USING (tenant_id = public.sesame_current_tenant_id())
WITH CHECK (tenant_id = public.sesame_current_tenant_id());

ALTER TABLE ar_agings ENABLE ROW LEVEL SECURITY;
ALTER TABLE ar_agings FORCE ROW LEVEL SECURITY;
CREATE POLICY ar_agings_tenant ON ar_agings
FOR ALL TO PUBLIC
USING (tenant_id = public.sesame_current_tenant_id())
WITH CHECK (tenant_id = public.sesame_current_tenant_id());

COMMIT;
\endif
