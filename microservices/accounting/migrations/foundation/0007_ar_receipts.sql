-- AR receipts (Epic F2a): the external payment reference (check number,
-- wire reference) is the authoritative duplicate guard for receipts, so it
-- must be unique within its tenant + legal entity scope. NULL references
-- stay permitted — a cash receipt can legitimately arrive without an
-- external identity until F2 allocations reconcile it.

-- The unique index itself is the completion marker for this migration.
-- A fresh application is transactional; a completed application is skipped.
SELECT EXISTS (
    SELECT 1
    FROM pg_indexes
    WHERE schemaname = current_schema()
      AND tablename = 'ar_payments'
      AND indexname = 'ar_payments_reference_unique'
) AS ar_payment_reference_unique_installed \gset

\if :ar_payment_reference_unique_installed
\echo 'AR payment reference unique index already installed; skipping 0007.'
\else
BEGIN;

CREATE UNIQUE INDEX ar_payments_reference_unique
    ON ar_payments (tenant_id, legal_entity_id, payment_reference)
    WHERE payment_reference IS NOT NULL;

COMMIT;
\endif
