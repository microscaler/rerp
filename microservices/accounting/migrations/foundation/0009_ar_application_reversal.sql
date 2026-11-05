-- Epic F2c: reversal returns a pair row's applied amount to zero IN PLACE.
-- The pair row is the (payment, document) anchor that the event journal
-- references; deleting it would CASCADE away the audit trail. So the
-- 0006 inline CHECK (applied_amount > 0) is relaxed to >= 0 and gains a
-- name. applied_amount = 0 means "currently not applied" (never applied
-- rows are never stored; reversal is the only way to reach 0). Re-apply
-- accumulates from 0 and is journaled as a fresh APPLIED event.
--
-- Online-safe: the new constraint is added NOT VALID (no full-table scan
-- under ACCESS EXCLUSIVE; new writes are checked immediately) and the
-- historical rows are validated in a separate step under the weaker
-- SHARE UPDATE EXCLUSIVE. The completion marker is the constraint being
-- present AND validated, so a rerun after a partial application still
-- finishes the validation.

SELECT EXISTS (
    SELECT 1
    FROM pg_constraint c
    JOIN pg_namespace n ON n.oid = c.connamespace
    WHERE c.conname = 'ar_payment_applications_applied_reversible_check'
      AND n.nspname = current_schema()
      AND c.convalidated
) AS ar_reversal_complete,
EXISTS (
    SELECT 1
    FROM pg_constraint c
    JOIN pg_namespace n ON n.oid = c.connamespace
    WHERE c.conname = 'ar_payment_applications_applied_reversible_check'
      AND n.nspname = current_schema()
) AS ar_reversal_added \gset

\if :ar_reversal_complete
\echo 'AR reversal constraint already installed and validated; skipping 0009.'
\else
\if :ar_reversal_added
\echo 'AR reversal constraint present but unvalidated; validating now.'
\else
BEGIN;

ALTER TABLE ar_payment_applications
    DROP CONSTRAINT ar_payment_applications_applied_amount_check;
ALTER TABLE ar_payment_applications
    ADD CONSTRAINT ar_payment_applications_applied_reversible_check
    CHECK (applied_amount >= 0) NOT VALID;

COMMIT;
\endif
ALTER TABLE ar_payment_applications
    VALIDATE CONSTRAINT ar_payment_applications_applied_reversible_check;
\endif
