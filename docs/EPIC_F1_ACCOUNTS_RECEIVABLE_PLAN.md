# Epic F1 — AR customer ledger from posted facts: slice plan

**Status: ACTIVE (F1a + F1b + F1c delivered; next: F2 receipts +
allocations).**
Depends on: D3 (posting/reversal), B1, C1 — all merged. Entry point stays
`microservices/accounting/BUILDOUT_ANALYSIS.md`; this file carries the
slice breakdown and evidence for F1.

## Ground truth (recon, main @ 09b76d10)

- AR's 4 models are scaffold LifeModels in the **impl** crate
  (`microservices/accounting/accounts-receivable/impl/src/models/`):
  `customer_invoices`, `ar_payments`, `ar_payment_applications`,
  `ar_agings`. No migrations existed; the scaffolds FK'd phantom
  `invoices`/`customers` tables and used NUMERIC(19,4).
- AR contract: 9 ops, all controllers stub-501, all 9 registry arms
  present. Two read ops (`list_open_items`, `list_customer_payments`)
  lack `x-brrtrouter-impl` while their controllers exist (legacy flag
  mode — set to true when implemented).
- Money gate: AR is in the STRICT set
  (`tooling/tests/test_accounting_money_types.py`).
- Posted facts: `accounting_posted_documents` (+ lines, journals,
  idempotency) with `original_document_id` credit/debit linkage (no FK,
  app-trusted), NUMERIC(19,6), FORCE-RLS via `sesame_current_tenant_id()`.
  Read-model precedent: read-time derivation, no SQL views
  (`general-ledger/impl/src/ledger.rs` trial balance).

## Slices (one PR each)

### F1a — entity rework + overlay migration + RLS (DONE 2026-11-01, PR #134)
- Models re-anchored: `invoice_id` → posted documents via tenant-scoped
  composite FK; plain `customer_id`; tenant/legal-entity columns;
  NUMERIC(19,6); scaffold drift columns dropped (days_overdue,
  aging_bucket, credit_limit/credit_used, collection_status) — aging is
  derived at read (F1b) and snapshotted in `ar_agings` (F3).
- `migrations/foundation/0006_ar_overlay.sql`: 4 tables, tenant policies
  as completion marker + guarded transaction (re-apply verified: skip),
  composite-scope FKs, CHECKs for status/terms/enums. Appended to
  `apply_order.txt`.
- Live acceptance (`impl/tests/ar_overlay_rls.rs`, `#[ignore]`):
  cross-tenant invisibility for all 4 tables through `rls_set_session`
  (session GUC is transaction-local — reads must run inside BEGIN),
  no-session sees nothing and cannot write (WITH CHECK), cross-tenant
  document anchor rejected by the composite FK; run-unique fixture ids
  make reruns safe (verified twice).

### F1b — open items from posted facts (DONE 2026-11-01, partial epic: 2 of 9 ops live)
- `list_open_items` + `get_open_item` live in the AR runtime (was honest 501):
  outstanding = Σ(INVOICE + DEBIT_NOTE) − Σ(CREDIT_NOTE linked via
  original_document_id) − Σ(F2 allocations) − overlay write-offs;
  aging bucket/days_overdue/terms derived at read; `company_id` filter or
  org-resolved legal entity; search over document number/status (customer
  name is not an AR-owned fact — documented deviation from the blurb).
- Runtime gained the invoice-shaped database/identity plumbing
  (pool env config, fail-fast init, `rls_set_session` context).
- Error bodies speak the contract's `ErrorResponse` enum codes
  (NOT_FOUND/FORBIDDEN/SERVER_ERROR/UNKNOWN) — the response validator
  rejects anything else with a 500 wrap.
- OpenItem id IS the posted document id (stable in list and get).
- Spec: `x-brrtrouter-impl: true` on `list_open_items`; dead examples
  (`active`, `sample value`) replaced with enum-valid values across the
  AR spec; BFF aggregate + gen regenerated.
- Proof: 8 unit tests; live-DB derivation test (credits netted, PARTIAL
  status, filters, payment_history, cross-tenant NotFound); live HTTP
  smoke on the release binary (200 list/get, 404 NOT_FOUND, 400 enum
  codes, search, limit-range rejection).
- Remaining AR ops stay honest 501: payments/applications (F2), aging
  summary + statements (F3).

### F1c — overlay projection + agreement suite (DONE 2026-11-02, PR #141)
- `projection.rs` (impl): `project_settlement_state` materializes the F1b
  derivation into `customer_invoices` — one row per posted customer
  document, values equal to `compose_item` (cached `original_amount` is
  the CREDIT-NETTED original, matching the read path). Idempotent
  (re-run writes nothing); LOSSLESS (manual WRITTEN_OFF / DISPUTED rows
  are never rewritten).
- `ar_control_drift`: journal-line movement on the ACCOUNTS_RECEIVABLE
  control account minus signed posted-document totals; `Ok(None)` when
  unconfigured or over the read bounds (degrades, never fails a
  projection).
- Live acceptance (`projection::tests`, ignored, disposable DB): run 1
  creates 2, refreshes 1 stale row, preserves 1 manual row, drift 0;
  cached dump agrees value-for-value with `list_open_items` over the
  same facts (manual row: read honors WRITTEN_OFF status while money
  stays derived from facts, cache frozen); run 2 is a no-op; a
  from-scratch rebuild (all derived rows deleted) is byte-identical to
  the first build; a stray control-account debit reads as drift +7.
  Tenant B's document invisible throughout (documents stays 4).
- The runtime consumer arrives with F3 (aging snapshots); module carries
  a documented `allow(dead_code)` until then — acceptance test is the
  caller, same proof-first pattern as F1b.

## Risks carried per PR
- Derived truth stays in the posted facts; the overlay is cache-like and
  must always be provably rebuildable (F1c suite).
- Credit subtraction scoped to tenant+legal_entity (no FK on the link).
- Registry atomicity for F1b; gen from the pinned BRRTRouter checkout.
- `apply_order.txt` append-only + guarded idempotent DDL.

- Deferred sweep: placeholder `example: 1000.0` / `bank_transfer` values sit in
  scaffold specs repo-wide (AR, AP, treasury, documents-extraction, ...); they
  predate F1 and need a dedicated all-specs cleanup PR, not an epic-scoped edit.
