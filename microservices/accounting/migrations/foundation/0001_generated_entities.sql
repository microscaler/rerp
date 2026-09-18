-- Migration: Generated from Lifeguard entities
-- Provider: accounting/foundation
-- Version: 20260714190206
-- Generated: 2026-07-14 19:02:06 UTC

-- This migration was automatically generated from entity definitions.
-- DO NOT EDIT MANUALLY - regenerate from entities instead.

-- Table: accounting_idempotency_records
CREATE TABLE IF NOT EXISTS accounting_idempotency_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR(200) NOT NULL,
    legal_entity_id UUID NOT NULL,
    idempotency_key VARCHAR(200) NOT NULL,
    request_fingerprint VARCHAR(64) NOT NULL,
    status VARCHAR(20) NOT NULL,
    source_system VARCHAR(100) NOT NULL,
    source_type VARCHAR(100) NOT NULL,
    source_id VARCHAR(255) NOT NULL,
    document_id UUID,
    journal_entry_id UUID,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP,
    UNIQUE(tenant_id, legal_entity_id, idempotency_key)
);

CREATE INDEX IF NOT EXISTS idx_accounting_idempotency_scope ON accounting_idempotency_records(tenant_id, legal_entity_id);
CREATE INDEX IF NOT EXISTS idx_accounting_idempotency_source ON accounting_idempotency_records(source_system, source_type, source_id);
CREATE INDEX IF NOT EXISTS idx_accounting_idempotency_records_tenant_id ON accounting_idempotency_records(tenant_id);
CREATE INDEX IF NOT EXISTS idx_accounting_idempotency_records_legal_entity_id ON accounting_idempotency_records(legal_entity_id);
COMMENT ON TABLE accounting_idempotency_records IS 'Accounting command idempotency and payload conflict records';


-- Table: accounting_legal_entities
CREATE TABLE IF NOT EXISTS accounting_legal_entities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR(200) NOT NULL,
    organization_id UUID NOT NULL,
    legal_name VARCHAR(255) NOT NULL,
    base_currency VARCHAR(3) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, organization_id),
    UNIQUE(tenant_id, id)
);

CREATE INDEX IF NOT EXISTS idx_accounting_legal_entities_tenant ON accounting_legal_entities(tenant_id);
CREATE INDEX IF NOT EXISTS idx_accounting_legal_entities_organization ON accounting_legal_entities(organization_id);
COMMENT ON TABLE accounting_legal_entities IS 'Tenant-scoped legal entities owning accounting books';


-- Table: accounting_accounts
CREATE TABLE IF NOT EXISTS accounting_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR(200) NOT NULL,
    legal_entity_id UUID NOT NULL REFERENCES accounting_legal_entities(id) ON DELETE RESTRICT,
    code VARCHAR(50) NOT NULL,
    name VARCHAR(255) NOT NULL,
    account_type VARCHAR(20) NOT NULL,
    normal_side VARCHAR(6) NOT NULL,
    control_role VARCHAR(50),
    currency_code VARCHAR(3) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, legal_entity_id, code),
    UNIQUE(tenant_id, legal_entity_id, id)
);

CREATE INDEX IF NOT EXISTS idx_accounting_accounts_scope ON accounting_accounts(tenant_id, legal_entity_id);
CREATE INDEX IF NOT EXISTS idx_accounting_accounts_type ON accounting_accounts(account_type);
CREATE INDEX IF NOT EXISTS idx_accounting_accounts_control_role ON accounting_accounts(control_role);
CREATE INDEX IF NOT EXISTS idx_accounting_accounts_tenant_id ON accounting_accounts(tenant_id);
CREATE INDEX IF NOT EXISTS idx_accounting_accounts_legal_entity_id ON accounting_accounts(legal_entity_id);
COMMENT ON TABLE accounting_accounts IS 'Posting accounts for the controlled accounting core';


-- Table: accounting_fiscal_periods
CREATE TABLE IF NOT EXISTS accounting_fiscal_periods (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR(200) NOT NULL,
    legal_entity_id UUID NOT NULL REFERENCES accounting_legal_entities(id) ON DELETE RESTRICT,
    name VARCHAR(100) NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    state VARCHAR(20) NOT NULL DEFAULT 'OPEN',
    closed_at TIMESTAMP,
    closed_by UUID,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_valid_period_dates CHECK (start_date <= end_date),
    UNIQUE(tenant_id, legal_entity_id, name),
    UNIQUE(tenant_id, legal_entity_id, id)
);

CREATE INDEX IF NOT EXISTS idx_accounting_fiscal_periods_scope ON accounting_fiscal_periods(tenant_id, legal_entity_id);
CREATE INDEX IF NOT EXISTS idx_accounting_fiscal_periods_dates ON accounting_fiscal_periods(start_date, end_date);
CREATE INDEX IF NOT EXISTS idx_accounting_fiscal_periods_state ON accounting_fiscal_periods(state);
CREATE INDEX IF NOT EXISTS idx_accounting_fiscal_periods_tenant_id ON accounting_fiscal_periods(tenant_id);
CREATE INDEX IF NOT EXISTS idx_accounting_fiscal_periods_legal_entity_id ON accounting_fiscal_periods(legal_entity_id);
CREATE INDEX IF NOT EXISTS idx_accounting_fiscal_periods_start_date ON accounting_fiscal_periods(start_date);
CREATE INDEX IF NOT EXISTS idx_accounting_fiscal_periods_end_date ON accounting_fiscal_periods(end_date);
COMMENT ON TABLE accounting_fiscal_periods IS 'Tenant and legal-entity scoped fiscal posting periods';


-- Table: accounting_posted_documents
CREATE TABLE IF NOT EXISTS accounting_posted_documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR(200) NOT NULL,
    legal_entity_id UUID NOT NULL REFERENCES accounting_legal_entities(id) ON DELETE RESTRICT,
    fiscal_period_id UUID NOT NULL REFERENCES accounting_fiscal_periods(id) ON DELETE RESTRICT,
    document_number VARCHAR(100) NOT NULL,
    document_type VARCHAR(30) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'POSTED',
    original_document_id UUID,
    customer_id UUID NOT NULL,
    source_system VARCHAR(100) NOT NULL,
    source_type VARCHAR(100) NOT NULL,
    source_id VARCHAR(255) NOT NULL,
    document_date DATE NOT NULL,
    due_date DATE NOT NULL,
    currency_code VARCHAR(3) NOT NULL,
    rounding_minor_units INTEGER NOT NULL DEFAULT 0,
    subtotal NUMERIC(19, 6) NOT NULL DEFAULT 0,
    discount_amount NUMERIC(19, 6) NOT NULL DEFAULT 0,
    tax_amount NUMERIC(19, 6) NOT NULL DEFAULT 0,
    total_amount NUMERIC(19, 6) NOT NULL DEFAULT 0,
    posted_at TIMESTAMP NOT NULL,
    posted_by UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, legal_entity_id, document_number),
    UNIQUE(tenant_id, legal_entity_id, source_system, source_type, source_id),
    UNIQUE(tenant_id, legal_entity_id, id)
);

CREATE INDEX IF NOT EXISTS idx_accounting_documents_scope ON accounting_posted_documents(tenant_id, legal_entity_id);
CREATE INDEX IF NOT EXISTS idx_accounting_documents_customer ON accounting_posted_documents(customer_id);
CREATE INDEX IF NOT EXISTS idx_accounting_documents_source ON accounting_posted_documents(source_system, source_type, source_id);
CREATE INDEX IF NOT EXISTS idx_accounting_documents_period ON accounting_posted_documents(fiscal_period_id);
CREATE INDEX IF NOT EXISTS idx_accounting_posted_documents_tenant_id ON accounting_posted_documents(tenant_id);
CREATE INDEX IF NOT EXISTS idx_accounting_posted_documents_legal_entity_id ON accounting_posted_documents(legal_entity_id);
COMMENT ON TABLE accounting_posted_documents IS 'Immutable posted accounting document headers';


-- Table: accounting_journal_entries
CREATE TABLE IF NOT EXISTS accounting_journal_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR(200) NOT NULL,
    legal_entity_id UUID NOT NULL,
    fiscal_period_id UUID NOT NULL REFERENCES accounting_fiscal_periods(id) ON DELETE RESTRICT,
    entry_number VARCHAR(100) NOT NULL,
    entry_date DATE NOT NULL,
    source_document_id UUID NOT NULL REFERENCES accounting_posted_documents(id) ON DELETE RESTRICT,
    currency_code VARCHAR(3) NOT NULL,
    total_debit NUMERIC(19, 6) NOT NULL DEFAULT 0,
    total_credit NUMERIC(19, 6) NOT NULL DEFAULT 0,
    posted_at TIMESTAMP NOT NULL,
    posted_by UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_balanced_journal CHECK (total_debit = total_credit),
    UNIQUE(tenant_id, legal_entity_id, entry_number),
    UNIQUE(tenant_id, legal_entity_id, source_document_id),
    UNIQUE(tenant_id, legal_entity_id, id)
);

CREATE INDEX IF NOT EXISTS idx_accounting_journal_scope ON accounting_journal_entries(tenant_id, legal_entity_id);
CREATE INDEX IF NOT EXISTS idx_accounting_journal_period ON accounting_journal_entries(fiscal_period_id);
CREATE INDEX IF NOT EXISTS idx_accounting_journal_date ON accounting_journal_entries(entry_date);
CREATE INDEX IF NOT EXISTS idx_accounting_journal_entries_tenant_id ON accounting_journal_entries(tenant_id);
CREATE INDEX IF NOT EXISTS idx_accounting_journal_entries_legal_entity_id ON accounting_journal_entries(legal_entity_id);
COMMENT ON TABLE accounting_journal_entries IS 'Immutable posted journal entry headers';


-- Table: accounting_journal_lines
CREATE TABLE IF NOT EXISTS accounting_journal_lines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR(200) NOT NULL,
    legal_entity_id UUID NOT NULL,
    journal_entry_id UUID NOT NULL REFERENCES accounting_journal_entries(id) ON DELETE RESTRICT,
    line_number INTEGER NOT NULL DEFAULT 0,
    account_id UUID NOT NULL REFERENCES accounting_accounts(id) ON DELETE RESTRICT,
    side VARCHAR(6) NOT NULL,
    amount NUMERIC(19, 6) NOT NULL DEFAULT 0,
    description VARCHAR(1000) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_one_sided_amount CHECK ((side = 'DEBIT' OR side = 'CREDIT') AND amount > 0),
    UNIQUE(tenant_id, legal_entity_id, journal_entry_id, line_number)
);

CREATE INDEX IF NOT EXISTS idx_accounting_journal_lines_scope ON accounting_journal_lines(tenant_id, legal_entity_id);
CREATE INDEX IF NOT EXISTS idx_accounting_journal_lines_entry ON accounting_journal_lines(journal_entry_id);
CREATE INDEX IF NOT EXISTS idx_accounting_journal_lines_account ON accounting_journal_lines(account_id);
CREATE INDEX IF NOT EXISTS idx_accounting_journal_lines_tenant_id ON accounting_journal_lines(tenant_id);
CREATE INDEX IF NOT EXISTS idx_accounting_journal_lines_legal_entity_id ON accounting_journal_lines(legal_entity_id);
COMMENT ON TABLE accounting_journal_lines IS 'Immutable one-sided lines of posted journals';


-- Table: accounting_posted_document_lines
CREATE TABLE IF NOT EXISTS accounting_posted_document_lines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR(200) NOT NULL,
    legal_entity_id UUID NOT NULL,
    document_id UUID NOT NULL REFERENCES accounting_posted_documents(id) ON DELETE RESTRICT,
    line_number INTEGER NOT NULL DEFAULT 0,
    description VARCHAR(1000) NOT NULL,
    quantity NUMERIC(19, 6) NOT NULL DEFAULT 0,
    unit_price NUMERIC(19, 6) NOT NULL DEFAULT 0,
    discount_percent NUMERIC(9, 6) NOT NULL DEFAULT 0,
    gross_amount NUMERIC(19, 6) NOT NULL DEFAULT 0,
    discount_amount NUMERIC(19, 6) NOT NULL DEFAULT 0,
    net_amount NUMERIC(19, 6) NOT NULL DEFAULT 0,
    tax_code VARCHAR(100),
    tax_rate_percent NUMERIC(9, 6),
    tax_amount NUMERIC(19, 6) NOT NULL DEFAULT 0,
    total_amount NUMERIC(19, 6) NOT NULL DEFAULT 0,
    revenue_account_id UUID NOT NULL REFERENCES accounting_accounts(id) ON DELETE RESTRICT,
    tax_liability_account_id UUID,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, legal_entity_id, document_id, line_number)
);

CREATE INDEX IF NOT EXISTS idx_accounting_document_lines_scope ON accounting_posted_document_lines(tenant_id, legal_entity_id);
CREATE INDEX IF NOT EXISTS idx_accounting_document_lines_document ON accounting_posted_document_lines(document_id);
CREATE INDEX IF NOT EXISTS idx_accounting_posted_document_lines_tenant_id ON accounting_posted_document_lines(tenant_id);
CREATE INDEX IF NOT EXISTS idx_accounting_posted_document_lines_legal_entity_id ON accounting_posted_document_lines(legal_entity_id);
COMMENT ON TABLE accounting_posted_document_lines IS 'Immutable line snapshots for posted accounting documents';


-- Table: accounting_audit_events
CREATE TABLE IF NOT EXISTS accounting_audit_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR(200) NOT NULL,
    legal_entity_id UUID NOT NULL,
    subject_id UUID NOT NULL,
    action VARCHAR(50) NOT NULL,
    document_id UUID NOT NULL,
    original_document_id UUID,
    source_system VARCHAR(100) NOT NULL,
    request_fingerprint VARCHAR(64) NOT NULL,
    occurred_at TIMESTAMP NOT NULL,
    recorded_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_accounting_audit_scope ON accounting_audit_events(tenant_id, legal_entity_id);
CREATE INDEX IF NOT EXISTS idx_accounting_audit_document ON accounting_audit_events(document_id);
CREATE INDEX IF NOT EXISTS idx_accounting_audit_time ON accounting_audit_events(occurred_at);
CREATE INDEX IF NOT EXISTS idx_accounting_audit_events_tenant_id ON accounting_audit_events(tenant_id);
CREATE INDEX IF NOT EXISTS idx_accounting_audit_events_legal_entity_id ON accounting_audit_events(legal_entity_id);
COMMENT ON TABLE accounting_audit_events IS 'Append-only accounting transition audit facts';
