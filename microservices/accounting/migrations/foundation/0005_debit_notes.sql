-- Customer debit notes (Epic E2b): a new posted-document type charging
-- additional quantities against an original customer invoice. The type
-- CHECK authored by 0002 is widened to accept CUSTOMER_DEBIT_NOTE; no other
-- constraint changes.

-- The widened constraint definition is the completion marker for this
-- migration. A fresh application is transactional; a completed application
-- is skipped, so re-running the foundation sequence is idempotent.
SELECT EXISTS (
    SELECT 1
    FROM pg_constraint c
    JOIN pg_class t ON t.oid = c.conrelid
    WHERE t.relname = 'accounting_posted_documents'
      AND c.conname = 'accounting_documents_type_check'
      AND pg_get_constraintdef(c.oid) LIKE '%CUSTOMER_DEBIT_NOTE%'
) AS accounting_debit_note_type_installed \gset

\if :accounting_debit_note_type_installed
\echo 'Customer debit note document type already installed; skipping 0005.'
\else
BEGIN;

ALTER TABLE accounting_posted_documents
    DROP CONSTRAINT accounting_documents_type_check,
    ADD CONSTRAINT accounting_documents_type_check
    CHECK (document_type IN ('CUSTOMER_INVOICE', 'CUSTOMER_CREDIT_NOTE', 'CUSTOMER_DEBIT_NOTE'));

COMMIT;
\endif
