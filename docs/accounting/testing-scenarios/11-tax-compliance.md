# Tax Compliance — Use-Case and Testing Scenarios

Date: 2026-10-05

- **Status**: PROPOSED
- **Owner**: accounting maintainers
- **Scope**: microservices/accounting/tax-compliance
- **Last reviewed**: 2026-10-05

Purpose: acceptance scenarios for tax periods, tax rules (jurisdiction +
effective dates), tax returns (validate/submit), tax payments, and tax audit
packs.

Related docs:

- [`standards/ifrs-reference.md`](standards/ifrs-reference.md) — international standards (IFRS/IAS).
- [`../rules-engines/tax-compliance-rules-engine.md`](../rules-engines/tax-compliance-rules-engine.md)
- [`../../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md)

## Standards reference

### IFRS (international)

| Standard | Relevance | URL |
|---|---|---|
| IAS 12 — Income Taxes | Corporate income tax, current and deferred tax | <https://www.ifrs.org/issued-standards/list-of-standards/ias-12-income-taxes/> |


Note: VAT/GST/withholding mechanics are jurisdiction rules; IFRS governs
only how income taxes hit the financial statements (IAS 12). The scenario
suite treats tax rules as **versioned jurisdiction data**, never hardcoded.

## Endpoint inventory (11 operations)

Tax periods (list/create); tax rules (list/create); tax returns
(list/create/validate/submit); tax payments (list/create); tax audit packs
(list/create).

## Use-case scenarios

### UC-TAX-01 — Tax period lifecycle

Given company C-1, jurisdiction `ID`, tax_type VAT, January 2026.
When `POST /tax-periods {company_id, jurisdiction_code: "ID", tax_type: VAT, start_date: 2026-01-01, end_date: 2026-01-31, due_date: 2026-02-28}`.
Then created OPEN; lifecycle OPEN → LOCKED (data frozen) → FILED (return
submitted) → CLOSED; a period with start > end is rejected.

### UC-TAX-02 — Effective-dated tax rules

Given rule R1 (VAT 11%, effective_from 2026-01-01) and rule R2 (VAT 12%,
effective_from 2026-02-01) for jurisdiction ID.
When a return is prepared for January and February.
Then January lines use 11% and February lines use 12% — rule selection is
by (jurisdiction, tax_type, transaction date) with versioning; overlapping
effective ranges for the same jurisdiction+type are rejected at creation.

### UC-TAX-03 — VAT return computation (worked numbers)

Given January sales (net) 100,000.00 at 11% and purchases (net) 60,000.00
at 11%.
When a VAT return is prepared.
Then output VAT 11,000.00, input VAT 6,600.00, total_due 4,400.00, and
every line explains its source records (invoice ids + rule version).

### UC-TAX-04 — Withholding and corporate income types

Given tax_type WITHHOLDING with rate 2% on service invoices and
tax_type CORPORATE_INCOME with rate 22%.
Then each return type computes against its own rule set; mixing a VAT rule
into a corporate return is rejected (type mismatch).

### UC-TAX-05 — Validate before submit

Given a draft return missing a required line source, or with a rule version
that expired before the period.
When `POST /tax-returns/{id}/validate`.
Then `valid=false` with `issues[]` naming each problem; a clean return
returns `valid=true` and empty issues; validation is a dry run — nothing is
submitted.

### UC-TAX-06 — Submit with declaration

Given a VALIDATED return.
When `POST /tax-returns/{id}/submit {submitted_by, declaration: true}`.
Then 202 with an `external_reference`; status SUBMITTED → ACCEPTED/REJECTED
from the authority adapter; submitting without declaration, or a return not
in READY_FOR_VALIDATION/VALIDATED, is rejected.

### UC-TAX-07 — Explain every return line

Given an ACCEPTED return.
Then each line can be traced to its source records (invoice/payment/GL
entries) and the tax-rule version used — required by the rules-engine
dossier and audit practice.

### UC-TAX-08 — Tax payment lifecycle

Given a return with total_due 4,400.00.
When `POST /tax-payments {return_id, amount: "4400.00"}`.
Then SCHEDULED → SENT → SETTLED (FAILED/CANCELLED alternatives); paying
more than total_due is rejected; multiple payments may settle in parts.

### UC-TAX-09 — Audit pack

Given a FILED return.
When `POST /tax-audit-packs {return_id}`.
Then DRAFT → GENERATING → READY with `artifact_count` = number of
evidence artifacts; the pack becomes immutable once READY (archival
snapshot), so a later data change cannot alter filed evidence.

### UC-TAX-10 — Boundary validation

Given page 0, limit 101, invalid jurisdiction_code format, rule rate outside
0–100, unknown tax_type.
Then each is rejected 400; defaults page=1 limit=20.

### UC-TAX-11 — Tenant isolation

Given companies C-1 (tenant A) and C-2 (tenant B) in the same jurisdiction.
Then tenant B sees only its own periods/returns/rules; cross-tenant return
id → 404.

### UC-TAX-12 — Contract-only status guard

Given the service is contract-only today.
Then endpoints return scaffold data; acceptance means the scenarios above
fail loudly until implemented.

## Test coverage matrix

All scenarios: **not yet implemented** — all controllers are stubs
(`create_tax_rule` returns `rate: 0.0`, `validate_tax_return` returns
`valid: false` unconditionally), no models, no migrations, no tests.

| Scenario | Existing test | Status |
|---|---|---|
| UC-TAX-01..12 | none | not yet implemented |

## Known drift / gaps

- Contract-only service; `TaxReturn.status` includes READY_FOR_VALIDATION/
  VALIDATED but no engine drives those transitions.
- Localization: jurisdiction adapters (Indonesia, Malaysia, Singapore) are
  extension points per the BDD backlog P3 — the core service must stay
  jurisdiction-agnostic.
