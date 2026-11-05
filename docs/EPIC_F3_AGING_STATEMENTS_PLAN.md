# Epic F3 — AR aging & statements: slice plan

Status: ACTIVE (planning). Parent: Epic F in
[`microservices/accounting/BUILDOUT_ANALYSIS.md`](../microservices/accounting/BUILDOUT_ANALYSIS.md).
Prereq delivered: Epic F2 COMPLETE (PRs #144–#147) — receipts, allocations,
reversal with journal, and the cross-axis identity acceptance suite; F1a–F1c
overlay/projection machinery (PRs #134, #135, #141).

## Ground truth (recon, main @ 20b2eec2)

- Contract: `GET /reports/aging-summary` (`AgingSummary`: `current`,
  `bucket_1_30/31_60/61_90/90_plus`, `total_outstanding`, `as_of_date`,
  `company_id`, optional `currency_code`; query `company_id`, `customer_id`,
  `as_of_date`) and `POST /customer-statements/send` (`StatementSendJob`:
  `QUEUED|SENDING|COMPLETED|FAILED`, 202-accepted). Both controllers are
  honest 501s (`x-brrtrouter-impl: true`, scaffold).
- Bucket semantics already exist per item: `ledger::aging_bucket(due_date,
  today)` over `days_overdue` — CURRENT / 1-30 / 31-60 / 61-90 / 90+. The
  summary must be the SAME derivation, aggregated, or the two read axes
  disagree.
- "Reproducible as-of" means the summary must be computed from POSTED FACTS
  with as-of filters (documents/credit notes `posted_at <= as_of` end-of-day,
  applications `applied_at <= as_of`), NEVER from the mutable overlay cache or
  from manual overlay states (F1c manual states are today-state, not
  history). Recomputing the same `as_of_date` twice must answer identically.
- Money is decimal strings per contract pattern; multi-currency must not be
  silently summed: when no explicit `currency_code` is selected, scope the
  summary to the legal entity's base currency and echo it; documents in other
  currencies are excluded (visible in open items, absent from this one-currency
  summary — call it out in the response description via spec commit).
- Agrees-with-subledger/GL AC: `total_outstanding` as of today must equal the
  open-items list sum (same tenant scope) and reconcile with the AR control
  account drift machinery (`projection::ar_control_drift`) when GL journal
  lines are seeded (F1c precedent for the seeding shape).
- Statements: `documents/render` is deliberately SCAFFOLD-ONLY (its README
  forbids wiring until persistence/sandboxed rendering/storage exist; ADR 002
  + PRD-008). The delivered precedent is the invoice Epic E1 durable render
  outbox (`accounting/invoice/impl/src/outbox.rs`): the business service owns
  the frozen typed render model + a durable instruction written in the SAME
  transaction as its state; the materialiser claims later; nothing depends on
  the render pipeline being up. F3 mirrors that exactly — AR owns a statement
  job + outbox instruction and reports `QUEUED` honestly; no fake delivery,
  no SENDING/COMPLETED without a real artifact.
- Spec defects to fix in F3b's spec commit (examples only, like F2a):
  `SendCustomerStatementsRequest.statement_date` example `1000.0`,
  `delivery_method` example `bank_transfer` (not in its own enum).

## Slices (one PR each)

### F3-plan — this document + BUILDOUT pointer + wiki log (2026-11-05)

### F3a — aging_summary report, fact-derived and as-of reproducible
- Implement `aging_summary` (read axis, no migration): resolve scope
  (company_id narrows within org, never escapes — read-path rule), reuse
  `chargeback_documents` + `credit_note_totals` with as-of `posted_at`
  filters and application sums filtered `applied_at <= as_of`; bucket by
  `ledger::aging_bucket(due_date, as_of)`; money via `rust_decimal`,
  emitted as contract decimal strings; group per (entity, currency).
- Guards: unknown company 404/empty per read-path precedent; malformed
  as_of 400. `customer_id` filter is RLS-bounded scope, not a trust
  boundary.
- Proof (units + live, disposable DB): bucket boundary units already exist —
  pin summary-vs-item-list value-for-value TODAY equality; as-of reproducibility
  (seed dated documents/payments; same query twice identical; older as_of
  excludes later facts); cross-currency documents excluded from base-currency
  summary; `total_outstanding` == open-items sum; with seeded GL control rows,
  agrees with `ar_control_drift == Some(0)` shape.

### F3b — statement jobs: durable request + frozen render model + outbox
- Spec commit (examples only, see defects above). Migration `0010`:
  `ar_statement_jobs` (id, tenant/entity, customer scope, statement_date,
  delivery_method, status QUEUED default, created/updated, RLS) — AR-owned,
  one owner. Outbox instruction model reuses the Epic E1 SHAPE (instruction
  row with PENDING/IN_PROGRESS/DONE + lease + error fields) as
  `ar_statement_render_instructions` carrying the FROZEN statement render
  model (as-of open items, applications, running balance) — AR owns the
  facts, Documents will own rendering when it exists.
- `POST /customer-statements/send` in one transaction: validate dates
  (statement_date not future beyond guard), validate delivery_method enum,
  job row + one instruction per customer with customers that HAVE balances
  as of the date; 202 with `StatementSendJob{status: QUEUED,
  statement_count}`. No new state machine lies: the job stays QUEUED until a
  real materialiser lands (explicitly out of scope; render scaffold must not
  be wired). `GET` for job status only if a contract op exists — if not,
  the job is inspected via DB in tests, not by inventing an endpoint.
- Proof: guards (empty customer_ids, invalid method 400; foreign customer
  ids 404-scoped invisibility), transactional atomicity (job + instructions
  commit together), rerun-safe (new job per accepted call), live seed of the
  frozen render model contents (value-for-value against the same-day aging
  summary).

### F3c — aging/statements acceptance suite (epic closer)
- Live cross-axis identity for the reporting epic: aging buckets sum ==
  open-items outstanding == (GL control movement when seeded); as-of
  replay equals a re-run; statement instructions freeze exactly what aging
  said for that customer/date; cross-tenant probes; CodeQL/psql lessons
  from #141/#146 hold (SQL via STDIN, no tainted ids in sinks).

## Risks carried per PR

- F3a: as-of reads must never touch the overlay cache — the temptation to
  reuse `overlay_state` for speed is the one bug this slice must not have;
  the live tests pin cache-independence (mutate cache, as-of answer stable).
- F3b: the contract has no statement-job GET op — resist endpoint invention;
  delivery statuses beyond QUEUED belong to the render epic, not here.
- F3c: multi-currency identity only closes per currency; do not mix.
