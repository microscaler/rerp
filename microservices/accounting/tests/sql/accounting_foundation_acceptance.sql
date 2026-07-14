\set ON_ERROR_STOP on

-- Run against a disposable database after applying the Sesame RLS contract and
-- both accounting foundation migrations. The connection must be allowed to
-- create/set this NOLOGIN verification role (normally a local test superuser).
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'rerp_phase1_test') THEN
        CREATE ROLE rerp_phase1_test NOLOGIN NOSUPERUSER NOBYPASSRLS;
    END IF;
END
$$;

GRANT USAGE ON SCHEMA public TO rerp_phase1_test;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO rerp_phase1_test;
GRANT EXECUTE ON FUNCTION public.rls_set_session(text, uuid, uuid, text, jsonb, jsonb, text, text)
TO rerp_phase1_test;
GRANT EXECUTE ON FUNCTION public.sesame_current_tenant_id() TO rerp_phase1_test;
GRANT EXECUTE ON FUNCTION public.accounting_reject_immutable_mutation() TO rerp_phase1_test;

SET ROLE rerp_phase1_test;

-- Context-free work fails closed.
DO $$
BEGIN
    IF (SELECT count(*) FROM accounting_legal_entities) <> 0 THEN
        RAISE EXCEPTION 'RLS must hide every tenant without context';
    END IF;
END
$$;

BEGIN;
SELECT public.rls_set_session(
    'alpha',
    'aaaaaaaa-0000-0000-0000-000000000001',
    'aaaaaaaa-0000-0000-0000-000000000002',
    'alpha-session',
    '["accountant"]'::jsonb,
    '["accounting:write"]'::jsonb,
    'service',
    'tenant'
);

INSERT INTO accounting_legal_entities (
    id, tenant_id, organization_id, legal_name, base_currency
) VALUES (
    'aaaaaaaa-1000-0000-0000-000000000001',
    'alpha',
    'aaaaaaaa-0000-0000-0000-000000000002',
    'Alpha Logistics Ltd',
    'GBP'
);

INSERT INTO accounting_fiscal_periods (
    id, tenant_id, legal_entity_id, name, start_date, end_date, state
) VALUES (
    'aaaaaaaa-2000-0000-0000-000000000001',
    'alpha',
    'aaaaaaaa-1000-0000-0000-000000000001',
    '2026-07',
    '2026-07-01',
    '2026-07-31',
    'OPEN'
);

INSERT INTO accounting_accounts (
    id, tenant_id, legal_entity_id, code, name, account_type, normal_side,
    control_role, currency_code
) VALUES
(
    'aaaaaaaa-3000-0000-0000-000000000001', 'alpha',
    'aaaaaaaa-1000-0000-0000-000000000001', '1100', 'Trade receivables',
    'ASSET', 'DEBIT', 'ACCOUNTS_RECEIVABLE', 'GBP'
),
(
    'aaaaaaaa-3000-0000-0000-000000000002', 'alpha',
    'aaaaaaaa-1000-0000-0000-000000000001', '4000', 'Freight revenue',
    'REVENUE', 'CREDIT', NULL, 'GBP'
),
(
    'aaaaaaaa-3000-0000-0000-000000000003', 'alpha',
    'aaaaaaaa-1000-0000-0000-000000000001', '2200', 'VAT payable',
    'LIABILITY', 'CREDIT', 'TAX_LIABILITY', 'GBP'
);

INSERT INTO accounting_posted_documents (
    id, tenant_id, legal_entity_id, fiscal_period_id, document_number,
    document_type, status, customer_id, source_system, source_type, source_id,
    document_date, due_date, currency_code, rounding_minor_units, subtotal,
    discount_amount, tax_amount, total_amount, posted_at, posted_by
) VALUES (
    'aaaaaaaa-4000-0000-0000-000000000001', 'alpha',
    'aaaaaaaa-1000-0000-0000-000000000001',
    'aaaaaaaa-2000-0000-0000-000000000001', 'INV-2026-000001',
    'CUSTOMER_INVOICE', 'POSTED',
    'aaaaaaaa-5000-0000-0000-000000000001', 'hauliage', 'delivery', '123',
    '2026-07-14', '2026-08-13', 'GBP', 2,
    251.10, 25.11, 45.20, 271.19,
    '2026-07-14 09:30:00', 'aaaaaaaa-0000-0000-0000-000000000001'
);

INSERT INTO accounting_posted_document_lines (
    id, tenant_id, legal_entity_id, document_id, line_number, description,
    quantity, unit_price, discount_percent, gross_amount, discount_amount,
    net_amount, tax_code, tax_rate_percent, tax_amount, total_amount,
    revenue_account_id, tax_liability_account_id
) VALUES (
    'aaaaaaaa-4100-0000-0000-000000000001', 'alpha',
    'aaaaaaaa-1000-0000-0000-000000000001',
    'aaaaaaaa-4000-0000-0000-000000000001', 1, 'Freight service',
    2, 125.55, 10, 251.10, 25.11, 225.99, 'VAT-20', 20, 45.20, 271.19,
    'aaaaaaaa-3000-0000-0000-000000000002',
    'aaaaaaaa-3000-0000-0000-000000000003'
);

INSERT INTO accounting_journal_entries (
    id, tenant_id, legal_entity_id, fiscal_period_id, entry_number, entry_date,
    source_document_id, currency_code, total_debit, total_credit, posted_at,
    posted_by
) VALUES (
    'aaaaaaaa-6000-0000-0000-000000000001', 'alpha',
    'aaaaaaaa-1000-0000-0000-000000000001',
    'aaaaaaaa-2000-0000-0000-000000000001', 'JRN-2026-000001', '2026-07-14',
    'aaaaaaaa-4000-0000-0000-000000000001', 'GBP', 271.19, 271.19,
    '2026-07-14 09:30:00', 'aaaaaaaa-0000-0000-0000-000000000001'
);

INSERT INTO accounting_journal_lines (
    id, tenant_id, legal_entity_id, journal_entry_id, line_number, account_id,
    side, amount, description
) VALUES
(
    'aaaaaaaa-6100-0000-0000-000000000001', 'alpha',
    'aaaaaaaa-1000-0000-0000-000000000001',
    'aaaaaaaa-6000-0000-0000-000000000001', 1,
    'aaaaaaaa-3000-0000-0000-000000000001', 'DEBIT', 271.19, 'Receivable'
),
(
    'aaaaaaaa-6100-0000-0000-000000000002', 'alpha',
    'aaaaaaaa-1000-0000-0000-000000000001',
    'aaaaaaaa-6000-0000-0000-000000000001', 2,
    'aaaaaaaa-3000-0000-0000-000000000002', 'CREDIT', 225.99, 'Revenue'
),
(
    'aaaaaaaa-6100-0000-0000-000000000003', 'alpha',
    'aaaaaaaa-1000-0000-0000-000000000001',
    'aaaaaaaa-6000-0000-0000-000000000001', 3,
    'aaaaaaaa-3000-0000-0000-000000000003', 'CREDIT', 45.20, 'VAT'
);

INSERT INTO accounting_idempotency_records (
    id, tenant_id, legal_entity_id, idempotency_key, request_fingerprint,
    status, source_system, source_type, source_id, document_id,
    journal_entry_id, completed_at
) VALUES (
    'aaaaaaaa-7000-0000-0000-000000000001', 'alpha',
    'aaaaaaaa-1000-0000-0000-000000000001', 'delivery:123:invoice:v1',
    repeat('a', 64), 'COMPLETED', 'hauliage', 'delivery', '123',
    'aaaaaaaa-4000-0000-0000-000000000001',
    'aaaaaaaa-6000-0000-0000-000000000001', '2026-07-14 09:30:00'
);

INSERT INTO accounting_audit_events (
    id, tenant_id, legal_entity_id, subject_id, action, document_id,
    source_system, request_fingerprint, occurred_at
) VALUES (
    'aaaaaaaa-8000-0000-0000-000000000001', 'alpha',
    'aaaaaaaa-1000-0000-0000-000000000001',
    'aaaaaaaa-0000-0000-0000-000000000001', 'INVOICE_POSTED',
    'aaaaaaaa-4000-0000-0000-000000000001', 'hauliage', repeat('a', 64),
    '2026-07-14 09:30:00'
);
COMMIT;

-- A second tenant may create the same human document number without collision.
BEGIN;
SELECT public.rls_set_session(
    'beta',
    'bbbbbbbb-0000-0000-0000-000000000001',
    'bbbbbbbb-0000-0000-0000-000000000002',
    'beta-session', '[]'::jsonb, '[]'::jsonb, 'service', 'tenant'
);
INSERT INTO accounting_legal_entities (
    id, tenant_id, organization_id, legal_name, base_currency
) VALUES (
    'bbbbbbbb-1000-0000-0000-000000000001', 'beta',
    'bbbbbbbb-0000-0000-0000-000000000002', 'Beta Freight Ltd', 'GBP'
);
COMMIT;

-- Tenant visibility, tenant write policy, scoped FK, accounting checks,
-- immutability and subtransaction rollback all fail closed.
BEGIN;
SELECT public.rls_set_session(
    'alpha',
    'aaaaaaaa-0000-0000-0000-000000000001',
    'aaaaaaaa-0000-0000-0000-000000000002',
    'alpha-verify', '[]'::jsonb, '[]'::jsonb, 'service', 'tenant'
);
DO $$
BEGIN
    IF (SELECT count(*) FROM accounting_legal_entities) <> 1 THEN
        RAISE EXCEPTION 'alpha must see exactly one legal entity';
    END IF;
    IF (SELECT count(*) FROM accounting_posted_documents) <> 1 THEN
        RAISE EXCEPTION 'alpha must see exactly one posted document';
    END IF;

    BEGIN
        INSERT INTO accounting_legal_entities (
            id, tenant_id, organization_id, legal_name, base_currency
        ) VALUES (
            'bbbbbbbb-1000-0000-0000-000000000099', 'beta',
            'bbbbbbbb-0000-0000-0000-000000000099', 'RLS bypass', 'GBP'
        );
        RAISE EXCEPTION 'cross-tenant insert unexpectedly succeeded';
    EXCEPTION WHEN insufficient_privilege THEN
        NULL;
    END;

    BEGIN
        INSERT INTO accounting_accounts (
            id, tenant_id, legal_entity_id, code, name, account_type,
            normal_side, currency_code
        ) VALUES (
            'aaaaaaaa-3000-0000-0000-000000000099', 'alpha',
            'bbbbbbbb-1000-0000-0000-000000000001', '9999', 'Cross tenant',
            'ASSET', 'DEBIT', 'GBP'
        );
        RAISE EXCEPTION 'tenant-inconsistent foreign key unexpectedly succeeded';
    EXCEPTION WHEN foreign_key_violation THEN
        NULL;
    END;

    BEGIN
        UPDATE accounting_posted_documents
        SET total_amount = 1
        WHERE id = 'aaaaaaaa-4000-0000-0000-000000000001';
        RAISE EXCEPTION 'posted document mutation unexpectedly succeeded';
    EXCEPTION WHEN SQLSTATE '55000' THEN
        NULL;
    END;

    BEGIN
        INSERT INTO accounting_journal_entries (
            id, tenant_id, legal_entity_id, fiscal_period_id, entry_number,
            entry_date, source_document_id, currency_code, total_debit,
            total_credit, posted_at, posted_by
        ) VALUES (
            'aaaaaaaa-6000-0000-0000-000000000099', 'alpha',
            'aaaaaaaa-1000-0000-0000-000000000001',
            'aaaaaaaa-2000-0000-0000-000000000001', 'BROKEN', '2026-07-14',
            'aaaaaaaa-4000-0000-0000-000000000001', 'GBP', 10, 9,
            '2026-07-14 10:00:00', 'aaaaaaaa-0000-0000-0000-000000000001'
        );
        RAISE EXCEPTION 'unbalanced journal unexpectedly succeeded';
    EXCEPTION WHEN check_violation THEN
        NULL;
    END;

    BEGIN
        INSERT INTO accounting_posted_documents (
            id, tenant_id, legal_entity_id, fiscal_period_id, document_number,
            document_type, status, customer_id, source_system, source_type,
            source_id, document_date, due_date, currency_code,
            rounding_minor_units, subtotal, discount_amount, tax_amount,
            total_amount, posted_at, posted_by
        ) VALUES (
            'aaaaaaaa-4000-0000-0000-000000000099', 'alpha',
            'aaaaaaaa-1000-0000-0000-000000000001',
            'aaaaaaaa-2000-0000-0000-000000000001', 'INV-ROLLBACK',
            'CUSTOMER_INVOICE', 'POSTED',
            'aaaaaaaa-5000-0000-0000-000000000099', 'hauliage', 'delivery',
            'rollback', '2026-07-14', '2026-07-14', 'GBP', 2,
            10, 0, 0, 10, '2026-07-14 10:00:00',
            'aaaaaaaa-0000-0000-0000-000000000001'
        );
        INSERT INTO accounting_journal_entries (
            id, tenant_id, legal_entity_id, fiscal_period_id, entry_number,
            entry_date, source_document_id, currency_code, total_debit,
            total_credit, posted_at, posted_by
        ) VALUES (
            'aaaaaaaa-6000-0000-0000-000000000098', 'alpha',
            'aaaaaaaa-1000-0000-0000-000000000001',
            'aaaaaaaa-2000-0000-0000-000000000001', 'ROLLBACK', '2026-07-14',
            'aaaaaaaa-4000-0000-0000-000000000099', 'GBP', 10, 9,
            '2026-07-14 10:00:00', 'aaaaaaaa-0000-0000-0000-000000000001'
        );
    EXCEPTION WHEN check_violation THEN
        NULL;
    END;
    IF EXISTS (
        SELECT 1 FROM accounting_posted_documents
        WHERE id = 'aaaaaaaa-4000-0000-0000-000000000099'
    ) THEN
        RAISE EXCEPTION 'failed posting left a partial document';
    END IF;
END
$$;
COMMIT;

RESET ROLE;
