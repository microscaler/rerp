# Epic F1 — AR customer ledger from posted facts: slice plan

**Status: ACTIVE (F1a delivered, F1b/F1c to follow). Date: 2026-11-01.**
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

### F1a — entity rework + overlay migration + RLS (this slice)
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

### F1b — open items from posted facts (next PR)
- `list_open_items` + `get_open_item` live: outstanding =
  Σ(INVOICE + DEBIT_NOTE) − Σ(CREDIT_NOTE linked via original_document_id),
  tenant+legal-entity scoped; aging bucket/days derived at read;
  `company_id` ↔ `resolve_legal_entity`; terms derived from the
  document/due-date gap (NET_15/30/45/60, DUE_ON_RECEIPT, else CUSTOM);
  `credit_limit_check` = null until customer credit data exists.
- Contract: add `x-brrtrouter-impl: true` to the two flagless read ops
  (+ regen, atomic commit). Fix OpenItem `status.example: active`
  (violates its own enum). BFF aggregate + gen regen LAST, after all
  spec edits.
- Acceptance: unit derivation combos + live ledger test through the real
  posting machinery.

### F1c — overlay projection + agreement suite
- Rebuildable projection populating `customer_invoices` from posted
  facts; reconciliation test vs from-scratch recompute; AR
  control-account agreement via journal lines. May fold into F1b.

## Risks carried per PR
- Derived truth stays in the posted facts; the overlay is cache-like and
  must always be provably rebuildable (F1c suite).
- Credit subtraction scoped to tenant+legal_entity (no FK on the link).
- Registry atomicity for F1b; gen from the pinned BRRTRouter checkout.
- `apply_order.txt` append-only + guarded idempotent DDL.
