# Lease Accounting — Use-Case and Testing Scenarios

Date: 2026-10-05

- **Status**: PROPOSED
- **Owner**: accounting maintainers
- **Scope**: microservices/accounting/lease-accounting
- **Last reviewed**: 2026-10-05

Purpose: acceptance scenarios for leases (classification), payment
schedules, lease liabilities, right-of-use assets, and lease modifications
(with recalculation and approval).

Related docs:

- [`standards/ifrs-reference.md`](standards/ifrs-reference.md) — international standards (IFRS/IAS).
- [`../rules-engines/lease-accounting-engine.md`](../rules-engines/lease-accounting-engine.md)
- [`../../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md)

## Standards reference

### IFRS (international)

| Standard | Relevance | URL |
|---|---|---|
| IFRS 16 — Leases | Single lessee model: ROU asset + lease liability; finance vs operating classification; modifications | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-16-leases/> |
| IAS 36 — Impairment of Assets | Impairment of ROU assets | <https://www.ifrs.org/issued-standards/list-of-standards/ias-36-impairment-of-assets/> |
| IAS 16 — Property, Plant and Equipment | ROU asset depreciation mechanics | <https://www.ifrs.org/issued-standards/list-of-standards/ias-16-property-plant-and-equipment/> |

## Endpoint inventory (9 operations)

Leases (list/create); lease payment schedules (list/create); lease
liabilities (list); right-of-use assets (list); lease modifications
(list/create/approve).

Classifications: OPERATING, FINANCE, SHORT_TERM, LOW_VALUE. Lease status:
DRAFT, ACTIVE, MODIFIED, TERMINATED, EXPIRED.

## Use-case scenarios

### UC-LSE-01 — Finance lease initial measurement

Given a 36-month equipment lease, payments 1,000.00/month, implicit rate 1%
monthly, commencement 2026-01-01.
When `POST /leases {lease_number: "L-001", classification: FINANCE, commencement_date: 2026-01-01, discount_rate: "0.01"}`.
Then the lease is ACTIVE and the liability/ROU are recognised at the
present value of the payment stream (≈ 30,107.50 at 1%/mo annuity factor
30.1075), not at the undiscounted 36,000.00.

### UC-LSE-02 — Short-term and low-value exemptions

Given leases classified SHORT_TERM (≤ 12 months, no purchase option) and
LOW_VALUE (under the threshold).
Then no ROU/liability recognition is required — the exemption is recorded
on the lease; a SHORT_TERM lease with a renewal option that makes the term
> 12 months must be reclassified to FINANCE/OPERATING (the exemption is
invalidated), per IFRS 16.5–8.

### UC-LSE-03 — Payment schedule generation

Given lease L-001.
When `POST /lease-payment-schedules {lease_id, payment_date: 2026-02-01, amount: "1000.00", currency_code: USD}`.
Then the schedule accumulates payments in order; a payment date before
commencement is rejected; each payment is allocable between interest
expense and liability reduction at the liability's carrying rate.

### UC-LSE-04 — Liability amortisation (worked numbers)

Given opening liability 30,107.50 at 1% monthly and a 1,000.00 payment.
Then interest = 301.08 (30,107.50 × 1%), principal = 698.92, closing
balance = 29,408.58; `opening_balance`/`closing_balance` on the liability
record reflect this; after 36 payments the closing balance is 0.00 within
rounding tolerance (no residual drift).

### UC-LSE-05 — ROU asset carrying value

Given lease L-001 with straight-line depreciation over 36 months.
Then ROU carrying value starts at the liability's initial measurement and
depreciates 1/36 per month; the ROU record links `asset_id` once the
corresponding `asset` service record exists (cross-service handoff).

### UC-LSE-06 — Lease modification (re-measurement)

Given lease L-001 ACTIVE with 24 of 36 months remaining.
When `POST /lease-modifications {lease_id, effective_date, reason: "floor space added", ...}` then `approve`.
Then the liability and ROU are re-measured at the effective date with the
new payment stream and the same discount rate (or a revised rate if the
modification changes the lease term — per IFRS 16.44–46); the lease status
becomes MODIFIED; approving a DRAFT modification on a TERMINATED lease is
rejected.

### UC-LSE-07 — Modification approval workflow

Given a DRAFT modification.
When approve → status APPROVED (then POSTED after recalculation);
cancelling a DRAFT modification is allowed; modifying an APPROVED
modification is rejected (versioned changes only).

### UC-LSE-08 — Termination and expiry

Given an ACTIVE lease.
When terminated early.
Then status TERMINATED with termination_date set and the remaining
liability/ROU derecognised (gain/loss computed); a lease whose term ends
naturally becomes EXPIRED.

### UC-LSE-09 — Operating leases still produce schedules

Given classification OPERATING.
Then the payment schedule exists (straight-line expense pattern), but no
ROU/liability rows are created for the lessee under the old-model choice —
RERP must store which model each lease uses so reports can switch between
IFRS 16 and legacy disclosure views.

### UC-LSE-10 — Boundary validation

Given page 0, limit 101, a payment amount ≤ 0, commencement date before
the payment date, an invalid classification enum.
Then each is rejected 400; defaults page=1 limit=20.

### UC-LSE-11 — Tenant isolation

Given leases of tenant A and tenant B with the same lease_number.
Then tenant B sees only its own leases/schedules/liabilities/ROUs;
cross-tenant lease id → 404.

### UC-LSE-12 — Explainability

Given any liability or ROU balance.
Then the service can explain it: payment stream, discount rate + version,
modifications applied, and period movements (interest/principal/
depreciation) — required by the engine dossier.

### UC-LSE-13 — Contract-only status guard

Given the service is contract-only today (stub controllers, read-only
liability/ROU lists).
Then scenarios fail loudly until implemented; the create/update endpoints
for liabilities/ROUs do not even exist yet (read-only lists only).

## Test coverage matrix

All scenarios: **not yet implemented** — contract-only service; all
controllers are stubs, no services/models, no tests, no migrations.

| Scenario | Existing test | Status |
|---|---|---|
| UC-LSE-01..13 | none | not yet implemented |

## Known drift / gaps

- Contract-only; `LeaseLiability`/`RightOfUseAsset` have **no create or
  update endpoints** (read-only lists), so UC-LSE-01/04/05 need contract
  additions.
- Spec examples are placeholder-quality (`payment_date: 1000.0` typed as a
  date) — spec cleanup required.
- Open design questions from the engine dossier (discount-rate sourcing and
  versioning, rule-based vs manual classification, ROU linkage to `asset`)
  remain open and are answered by these scenarios as acceptance targets.
