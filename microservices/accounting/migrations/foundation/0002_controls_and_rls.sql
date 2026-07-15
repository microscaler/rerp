-- Phase 1 Accounting foundation controls which cannot yet be expressed by Lifeguard's
-- entity migration metadata: tenant-consistent composite foreign keys,
-- immutability triggers and app-owned PostgreSQL RLS policy.
--
-- Prerequisite: apply sql/rls/v1/install.sql and grant its functions to the
-- runtime database role before application traffic is enabled.

ALTER TABLE accounting_accounts
    ADD CONSTRAINT accounting_accounts_legal_entity_scope_fk
    FOREIGN KEY (tenant_id, legal_entity_id)
    REFERENCES accounting_legal_entities(tenant_id, id)
    ON DELETE RESTRICT;

ALTER TABLE accounting_fiscal_periods
    ADD CONSTRAINT accounting_fiscal_periods_legal_entity_scope_fk
    FOREIGN KEY (tenant_id, legal_entity_id)
    REFERENCES accounting_legal_entities(tenant_id, id)
    ON DELETE RESTRICT;

ALTER TABLE accounting_posted_documents
    ADD CONSTRAINT accounting_documents_period_scope_fk
    FOREIGN KEY (tenant_id, legal_entity_id, fiscal_period_id)
    REFERENCES accounting_fiscal_periods(tenant_id, legal_entity_id, id)
    ON DELETE RESTRICT,
    ADD CONSTRAINT accounting_documents_original_scope_fk
    FOREIGN KEY (tenant_id, legal_entity_id, original_document_id)
    REFERENCES accounting_posted_documents(tenant_id, legal_entity_id, id)
    ON DELETE RESTRICT,
    ADD CONSTRAINT accounting_documents_currency_check
    CHECK (currency_code ~ '^[A-Z]{3}$'),
    ADD CONSTRAINT accounting_documents_type_check
    CHECK (document_type IN ('CUSTOMER_INVOICE', 'CUSTOMER_CREDIT_NOTE')),
    ADD CONSTRAINT accounting_documents_status_check
    CHECK (status = 'POSTED'),
    ADD CONSTRAINT accounting_documents_dates_check
    CHECK (due_date >= document_date),
    ADD CONSTRAINT accounting_documents_rounding_check
    CHECK (rounding_minor_units BETWEEN 0 AND 6),
    ADD CONSTRAINT accounting_documents_amounts_check
    CHECK (
        subtotal >= 0
        AND discount_amount >= 0
        AND discount_amount <= subtotal
        AND tax_amount >= 0
        AND total_amount > 0
        AND total_amount = subtotal - discount_amount + tax_amount
    );

ALTER TABLE accounting_posted_document_lines
    ADD CONSTRAINT accounting_document_lines_document_scope_fk
    FOREIGN KEY (tenant_id, legal_entity_id, document_id)
    REFERENCES accounting_posted_documents(tenant_id, legal_entity_id, id)
    ON DELETE RESTRICT,
    ADD CONSTRAINT accounting_document_lines_revenue_scope_fk
    FOREIGN KEY (tenant_id, legal_entity_id, revenue_account_id)
    REFERENCES accounting_accounts(tenant_id, legal_entity_id, id)
    ON DELETE RESTRICT,
    ADD CONSTRAINT accounting_document_lines_tax_scope_fk
    FOREIGN KEY (tenant_id, legal_entity_id, tax_liability_account_id)
    REFERENCES accounting_accounts(tenant_id, legal_entity_id, id)
    ON DELETE RESTRICT,
    ADD CONSTRAINT accounting_document_lines_values_check
    CHECK (
        line_number > 0
        AND quantity > 0
        AND unit_price >= 0
        AND discount_percent BETWEEN 0 AND 100
        AND gross_amount >= 0
        AND discount_amount >= 0
        AND discount_amount <= gross_amount
        AND net_amount = gross_amount - discount_amount
        AND tax_amount >= 0
        AND total_amount = net_amount + tax_amount
        AND (tax_rate_percent IS NULL OR tax_rate_percent BETWEEN 0 AND 100)
        AND ((tax_amount = 0) OR tax_liability_account_id IS NOT NULL)
    );

ALTER TABLE accounting_journal_entries
    ADD CONSTRAINT accounting_journals_period_scope_fk
    FOREIGN KEY (tenant_id, legal_entity_id, fiscal_period_id)
    REFERENCES accounting_fiscal_periods(tenant_id, legal_entity_id, id)
    ON DELETE RESTRICT,
    ADD CONSTRAINT accounting_journals_document_scope_fk
    FOREIGN KEY (tenant_id, legal_entity_id, source_document_id)
    REFERENCES accounting_posted_documents(tenant_id, legal_entity_id, id)
    ON DELETE RESTRICT,
    ADD CONSTRAINT accounting_journals_amount_check
    CHECK (total_debit > 0 AND total_credit > 0),
    ADD CONSTRAINT accounting_journals_currency_check
    CHECK (currency_code ~ '^[A-Z]{3}$');

ALTER TABLE accounting_journal_lines
    ADD CONSTRAINT accounting_journal_lines_journal_scope_fk
    FOREIGN KEY (tenant_id, legal_entity_id, journal_entry_id)
    REFERENCES accounting_journal_entries(tenant_id, legal_entity_id, id)
    ON DELETE RESTRICT,
    ADD CONSTRAINT accounting_journal_lines_account_scope_fk
    FOREIGN KEY (tenant_id, legal_entity_id, account_id)
    REFERENCES accounting_accounts(tenant_id, legal_entity_id, id)
    ON DELETE RESTRICT,
    ADD CONSTRAINT accounting_journal_lines_number_check
    CHECK (line_number > 0);

ALTER TABLE accounting_idempotency_records
    ADD CONSTRAINT accounting_idempotency_legal_entity_scope_fk
    FOREIGN KEY (tenant_id, legal_entity_id)
    REFERENCES accounting_legal_entities(tenant_id, id)
    ON DELETE RESTRICT,
    ADD CONSTRAINT accounting_idempotency_document_scope_fk
    FOREIGN KEY (tenant_id, legal_entity_id, document_id)
    REFERENCES accounting_posted_documents(tenant_id, legal_entity_id, id)
    ON DELETE RESTRICT,
    ADD CONSTRAINT accounting_idempotency_journal_scope_fk
    FOREIGN KEY (tenant_id, legal_entity_id, journal_entry_id)
    REFERENCES accounting_journal_entries(tenant_id, legal_entity_id, id)
    ON DELETE RESTRICT,
    ADD CONSTRAINT accounting_idempotency_status_check
    CHECK (status IN ('STARTED', 'COMPLETED'));

ALTER TABLE accounting_audit_events
    ADD CONSTRAINT accounting_audit_legal_entity_scope_fk
    FOREIGN KEY (tenant_id, legal_entity_id)
    REFERENCES accounting_legal_entities(tenant_id, id)
    ON DELETE RESTRICT,
    ADD CONSTRAINT accounting_audit_document_scope_fk
    FOREIGN KEY (tenant_id, legal_entity_id, document_id)
    REFERENCES accounting_posted_documents(tenant_id, legal_entity_id, id)
    ON DELETE RESTRICT,
    ADD CONSTRAINT accounting_audit_original_scope_fk
    FOREIGN KEY (tenant_id, legal_entity_id, original_document_id)
    REFERENCES accounting_posted_documents(tenant_id, legal_entity_id, id)
    ON DELETE RESTRICT;

ALTER TABLE accounting_accounts
    ADD CONSTRAINT accounting_accounts_currency_check
    CHECK (currency_code ~ '^[A-Z]{3}$'),
    ADD CONSTRAINT accounting_accounts_type_check
    CHECK (account_type IN ('ASSET', 'LIABILITY', 'EQUITY', 'REVENUE', 'EXPENSE')),
    ADD CONSTRAINT accounting_accounts_normal_side_check
    CHECK (normal_side IN ('DEBIT', 'CREDIT'));

ALTER TABLE accounting_legal_entities
    ADD CONSTRAINT accounting_legal_entities_currency_check
    CHECK (base_currency ~ '^[A-Z]{3}$');

ALTER TABLE accounting_fiscal_periods
    ADD CONSTRAINT accounting_fiscal_periods_state_check
    CHECK (state IN ('OPEN', 'CLOSED')),
    ADD CONSTRAINT accounting_fiscal_periods_close_fields_check
    CHECK (
        (state = 'OPEN' AND closed_at IS NULL AND closed_by IS NULL)
        OR (state = 'CLOSED' AND closed_at IS NOT NULL AND closed_by IS NOT NULL)
    );

CREATE OR REPLACE FUNCTION public.accounting_reject_immutable_mutation()
RETURNS trigger
LANGUAGE plpgsql
SECURITY INVOKER
SET search_path = pg_catalog
AS $$
BEGIN
    RAISE EXCEPTION 'posted accounting facts are immutable; use a credit note or reversal'
        USING ERRCODE = '55000';
END;
$$;

CREATE TRIGGER accounting_documents_immutable
BEFORE UPDATE OR DELETE ON accounting_posted_documents
FOR EACH ROW EXECUTE FUNCTION public.accounting_reject_immutable_mutation();

CREATE TRIGGER accounting_document_lines_immutable
BEFORE UPDATE OR DELETE ON accounting_posted_document_lines
FOR EACH ROW EXECUTE FUNCTION public.accounting_reject_immutable_mutation();

CREATE TRIGGER accounting_journals_immutable
BEFORE UPDATE OR DELETE ON accounting_journal_entries
FOR EACH ROW EXECUTE FUNCTION public.accounting_reject_immutable_mutation();

CREATE TRIGGER accounting_journal_lines_immutable
BEFORE UPDATE OR DELETE ON accounting_journal_lines
FOR EACH ROW EXECUTE FUNCTION public.accounting_reject_immutable_mutation();

CREATE TRIGGER accounting_audit_immutable
BEFORE UPDATE OR DELETE ON accounting_audit_events
FOR EACH ROW EXECUTE FUNCTION public.accounting_reject_immutable_mutation();

ALTER TABLE accounting_legal_entities ENABLE ROW LEVEL SECURITY;
ALTER TABLE accounting_legal_entities FORCE ROW LEVEL SECURITY;
CREATE POLICY accounting_legal_entities_tenant ON accounting_legal_entities
FOR ALL TO PUBLIC
USING (tenant_id = public.sesame_current_tenant_id())
WITH CHECK (tenant_id = public.sesame_current_tenant_id());

ALTER TABLE accounting_fiscal_periods ENABLE ROW LEVEL SECURITY;
ALTER TABLE accounting_fiscal_periods FORCE ROW LEVEL SECURITY;
CREATE POLICY accounting_fiscal_periods_tenant ON accounting_fiscal_periods
FOR ALL TO PUBLIC
USING (tenant_id = public.sesame_current_tenant_id())
WITH CHECK (tenant_id = public.sesame_current_tenant_id());

ALTER TABLE accounting_accounts ENABLE ROW LEVEL SECURITY;
ALTER TABLE accounting_accounts FORCE ROW LEVEL SECURITY;
CREATE POLICY accounting_accounts_tenant ON accounting_accounts
FOR ALL TO PUBLIC
USING (tenant_id = public.sesame_current_tenant_id())
WITH CHECK (tenant_id = public.sesame_current_tenant_id());

ALTER TABLE accounting_posted_documents ENABLE ROW LEVEL SECURITY;
ALTER TABLE accounting_posted_documents FORCE ROW LEVEL SECURITY;
CREATE POLICY accounting_posted_documents_tenant ON accounting_posted_documents
FOR ALL TO PUBLIC
USING (tenant_id = public.sesame_current_tenant_id())
WITH CHECK (tenant_id = public.sesame_current_tenant_id());

ALTER TABLE accounting_posted_document_lines ENABLE ROW LEVEL SECURITY;
ALTER TABLE accounting_posted_document_lines FORCE ROW LEVEL SECURITY;
CREATE POLICY accounting_posted_document_lines_tenant ON accounting_posted_document_lines
FOR ALL TO PUBLIC
USING (tenant_id = public.sesame_current_tenant_id())
WITH CHECK (tenant_id = public.sesame_current_tenant_id());

ALTER TABLE accounting_journal_entries ENABLE ROW LEVEL SECURITY;
ALTER TABLE accounting_journal_entries FORCE ROW LEVEL SECURITY;
CREATE POLICY accounting_journal_entries_tenant ON accounting_journal_entries
FOR ALL TO PUBLIC
USING (tenant_id = public.sesame_current_tenant_id())
WITH CHECK (tenant_id = public.sesame_current_tenant_id());

ALTER TABLE accounting_journal_lines ENABLE ROW LEVEL SECURITY;
ALTER TABLE accounting_journal_lines FORCE ROW LEVEL SECURITY;
CREATE POLICY accounting_journal_lines_tenant ON accounting_journal_lines
FOR ALL TO PUBLIC
USING (tenant_id = public.sesame_current_tenant_id())
WITH CHECK (tenant_id = public.sesame_current_tenant_id());

ALTER TABLE accounting_idempotency_records ENABLE ROW LEVEL SECURITY;
ALTER TABLE accounting_idempotency_records FORCE ROW LEVEL SECURITY;
CREATE POLICY accounting_idempotency_records_tenant ON accounting_idempotency_records
FOR ALL TO PUBLIC
USING (tenant_id = public.sesame_current_tenant_id())
WITH CHECK (tenant_id = public.sesame_current_tenant_id());

ALTER TABLE accounting_audit_events ENABLE ROW LEVEL SECURITY;
ALTER TABLE accounting_audit_events FORCE ROW LEVEL SECURITY;
CREATE POLICY accounting_audit_events_tenant ON accounting_audit_events
FOR ALL TO PUBLIC
USING (tenant_id = public.sesame_current_tenant_id())
WITH CHECK (tenant_id = public.sesame_current_tenant_id());
