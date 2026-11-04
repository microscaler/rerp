-- Epic F2c: reversal returns a pair row's applied amount to zero IN PLACE.
-- The pair row is the (payment, document) anchor that the event journal
-- references; deleting it would CASCADE away the audit trail. So the
-- 0006 inline CHECK (applied_amount > 0) is relaxed to >= 0 and gains a
-- name. applied_amount = 0 means "currently not applied" (never applied
-- rows are never stored; reversal is the only way to reach 0). Re-apply
-- accumulates from 0 and is journaled as a fresh APPLIED event.

-- The renamed constraint is the completion marker for this migration.
SELECT EXISTS (
    SELECT 1
    FROM pg_constraint c
    JOIN pg_namespace n ON n.oid = c.connamespace
    WHERE c.conname = 'ar_payment_applications_applied_reversible_check'
      AND n.nspname = current_schema()
) AS ar_reversal_installed \gset

\if :ar_reversal_installed
\echo 'AR reversal constraint already installed; skipping 0009.'
\else
BEGIN;

ALTER TABLE ar_payment_applications
    DROP CONSTRAINT ar_payment_applications_applied_amount_check;
ALTER TABLE ar_payment_applications
    ADD CONSTRAINT ar_payment_applications_applied_reversible_check
    CHECK (applied_amount >= 0);

COMMIT;
\endif
