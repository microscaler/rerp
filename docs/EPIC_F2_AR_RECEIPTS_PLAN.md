# Epic F2 — AR receipts + allocations: slice plan

Status: ACTIVE (planning). Parent: Epic F in
[`microservices/accounting/BUILDOUT_ANALYSIS.md`](../microservices/accounting/BUILDOUT_ANALYSIS.md).
Prereq delivered: F1a–F1c (PRs #134, #135, #141) — overlay tables, fact-derived
open items, and the overlay projection/agreement suite this epic builds on.

## Ground truth (recon, main @ d0465dc5)

- Contract: 8 payment-adjacent ops in
  `openapi/accounting/accounts-receivable/openapi.yaml` — `list_customer_payments`,
  `create_customer_payment` (idempotent, optional `auto_apply` FIFO),
  `get_customer_payment` (with application history), `list_payment_applications`,
  `create_payment_application` (partial supported). No reversal op exists in the
  contract, yet the epic AC requires *allocation reversal with audit linkage*
  (BUILDOUT Epic F2).
- Schema (migration 0006): `ar_payments` (status CHECK `DRAFT|POSTED|RECONCILED|CANCELLED`,
  `applied_amount`/`unapplied_amount` maintained, `payment_reference` nullable,
  `payment_number` NOT NULL with UNIQUE per tenant+entity) and
  `ar_payment_applications` (UNIQUE per payment+invoice pair — repeat
  applications must accumulate on the pair row; `applied_amount > 0` CHECK means
  reversals cannot live in this table).
- Contract↔DB vocabulary mismatches (contract is authority; mapping is ours):
  contract `reference_number` ↔ DB `payment_reference`; contract status enum
  (`PENDING|APPLIED|APPLIED_PARTIAL|REFUNDED|FAILED`) is a different axis from
  DB status — F1b precedent: derive the contract status from facts
  (see F2a); `applied_at` is derived (MAX application time when fully applied,
  else null — contract allows null).
- `posted_to_gl`/`gl_entry_id` are required in the contract `Payment` object but
  cash-side GL posting belongs to Epic H (banking). F2 returns
  `posted_to_gl=false`, `gl_entry_id=null` (overlay-level receipts only).
- Spec defects confined to the payment ops (fix in F2a's spec commit, examples
  only — no schema-affecting change): `payment_method`, `payment_date`, and the
  application-request `*_id` examples carry `1000.0` placeholders.

## Slices (one PR each)

### F2-plan — this document (+ BUILDOUT pointer + log) (2026-11-02)

### F2a — payments live (create / list / get)
- Migration `foundation/0007_ar_receipts.sql`: partial unique index
  `(tenant_id, legal_entity_id, payment_reference) WHERE payment_reference IS
  NOT NULL` — the epic's "authoritative external payment reference".
  Append-only to `apply_order.txt` (append-only rule; never renumber).
- `create_customer_payment`: server-generated `payment_number` (`PAY-<year>-
  <seq>` under the GL's advisory-lock sequence discipline), initial
  `applied=0`, `unapplied=payment_amount`, status `POSTED` (payments arrive
  as facts; DRAFT is unreachable via the API); duplicate external reference
  → 409 (the 0007 partial-unique index is the duplicate guard — the contract
  carries no request-level idempotency key, unlike GL, so key-bearing replay
  is tracked with F2c's contract additions). `auto_apply=true` is an
  honest 501 until F2b delivers the allocation machinery.
- Status derivation (contract axis): `applied=0` → `PENDING`;
  `0<applied<amount` → `APPLIED_PARTIAL`; `applied=amount` → `APPLIED`;
  DB `CANCELLED` → `FAILED`; `REFUNDED` reserved for Epic H.
- `company_id` in the request is optional and, when present, must equal the
  session legal entity (400 on mismatch) — same treatment F1b gave open items.
- Scope-class fix surfaced by the F2a fixture: unscoped `list`/`get` now
  cover ALL of the caller org's active entities (F1b's `resolve_legal_entity`
  picked an arbitrary one for multi-entity orgs), and a write in an ambiguous
  scope is rejected rather than guessed.

### F2b — applications live (create / list, auto_apply)
- Guards from F1b's fact-derived balance: application may not exceed the
  document's derived outstanding (409), nor the payment's remaining unapplied
  (409); a missing overlay row is HEALED through F1c's projection refresh (the
  application never hand-patches cache columns); payment must be internally
  `POSTED` (fresh F2a receipts qualify; CANCELLED rejected).
- Id space: the contract `customer_invoice_id` is the OPEN-ITEM (posted
  document) id — the one F1b echoes — while `ar_payment_applications.invoice_id`
  FKs the overlay row; the flow maps document→overlay internally and list
  re-maps back, so clients never see overlay row ids.
- Pair semantics: repeat application to the same (payment, invoice) accumulates
  into the UNIQUE pair row; every delta is journaled to the new
  `ar_payment_application_events` table (kind `APPLIED`, amount, actor, time) —
  this is also F2c's audit spine.
- Maintains in one transaction (advisory lock on the payment taken BEFORE any
  document lock — same FNV pattern as the F1c projection lock): payment
  `applied_amount`/`unapplied_amount`, overlay row refresh (manual states
  preserved per F1c rules; `PAID` transition derived), and the payment
  `applied_at` read-axis derivation (last allocation instant, batched per
  page — never stored).
- `auto_apply=true` on create = FIFO by due date over the customer's open items
  (same-currency, capacity-aware; nothing to allocate is a successful no-op).

### F2c — allocation reversal with audit linkage
- Contract addition (separate spec commit first, then regen):
  `POST /payments/applications/{id}/reverse` → reverses the pair row's applied
  amount in full, journaling a `REVERSED` event with actor + reason (audit
  linkage without violating the `applied_amount > 0` CHECK).
- Re-application after reversal resets the pair row and is journaled as a fresh
  `APPLIED` event (history lives in the events table).

### F2d — receipts acceptance suite
- Live (disposable-DB pattern from F1c, never the farm's scaled-down shared
  postgres): full/partial/multi-doc receipts; the identity
  `invoice + credit + receipt + allocation + outstanding` holds value-for-value
  across the read path, the cache, and the events journal; over-balance and
  over-payment applications rejected 409; reversal restores balances and leaves
  the audit trail; cross-tenant invisibility; rerun-safe idempotent replay.

## Risks carried per PR

- F2a: 501 on `auto_apply=true` is a visible intermediate state (contract says
  it works); call it out in the PR body.
- F2b: cache-vs-facts drift is the F1c lesson — the application flow must route
  through F1c's projection primitives, not hand-patch cache columns.
- F2c: contract additions need the regen ritual (`brrtrouter-openapi-codegen`
  conventions; `gen/` is disposable; behavior in `impl/`).
- F2d: live suites must keep the CodeQL/psql lessons from #141 (SQL via STDIN,
  no tainted ids in panic/log sinks, explicit stdout/stderr piping).
