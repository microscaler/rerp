# RERP Accounting — Use-Case and Testing Scenarios

Date: 2026-10-05

Purpose: one scenario file per accounting service, each containing
end-to-end use-case scenarios (Given/When/Then with worked numbers), the
international accounting standards (IFRS/IAS) each scenario must honour,
with official URLs — and a test-coverage matrix that maps every scenario to an existing
test or marks it as not yet implemented. This suite is the granular
explosion of the high-level themes in
[`ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md).

Related docs:

- [`standards/ifrs-reference.md`](standards/ifrs-reference.md) — international standards (IFRS/IAS), with official URLs.
- [`../rules-engines/README.md`](../rules-engines/README.md) — rules-engine design dossiers.
- [`../../DOCUMENTATION_GOVERNANCE.md`](../../DOCUMENTATION_GOVERNANCE.md) — document lifecycle rules.

## How to use this suite

- Each file is a **reviewable scenario reference**, not executable code.
  Scenarios are written so they can later be transcribed 1:1 into
  `#[cfg(test)]` unit tests, `tests/` integration tests, or Gherkin `.feature`
  files without design rework.
- Every scenario has an ID (`UC-<SV>-<NN>`). The Test Coverage Matrix in each
  file tracks that ID against the current implementation.
- **Status** in each file: `PROPOSED` until the maintainers accept it; scenario
  content is normative for QA once `APPROVED`.
- Language: English only (repository policy).

## Scenario inventory

| File | Service | Endpoints | Scenarios | Existing tests |
|---|---|---|---|---|
| [`01-general-ledger.md`](01-general-ledger.md) | general-ledger | 5 | 14 | 12 |
| [`02-invoice.md`](02-invoice.md) | invoice | 5 | 16 | 22 (1 `#[ignore]`) |
| [`03-accounts-payable.md`](03-accounts-payable.md) | accounts-payable | 20 | 15 | 8 |
| [`04-accounts-receivable.md`](04-accounts-receivable.md) | accounts-receivable | 29 | 16 | 12 |
| [`05-asset.md`](05-asset.md) | asset | 21 | 15 | 12 |
| [`06-bank-sync.md`](06-bank-sync.md) | bank-sync | 24 | 14 | 0 |
| [`07-budget.md`](07-budget.md) | budget | 17 | 13 | 0 |
| [`08-treasury.md`](08-treasury.md) | treasury | 9 | 10 | 0 |
| [`09-consolidation.md`](09-consolidation.md) | consolidation | 10 | 11 | 0 |
| [`10-financial-reports.md`](10-financial-reports.md) | financial-reports | 19 | 14 | 0 |
| [`11-tax-compliance.md`](11-tax-compliance.md) | tax-compliance | 11 | 12 | 0 |
| [`12-revenue-recognition.md`](12-revenue-recognition.md) | revenue-recognition | 10 | 12 | 0 |
| [`13-lease-accounting.md`](13-lease-accounting.md) | lease-accounting | 9 | 13 | 0 |
| [`14-documents-extraction.md`](14-documents-extraction.md) | documents-extraction | 9 | 11 | 0 |
| [`15-edi.md`](15-edi.md) | edi | 21 | 12 | 0 |
| [`16-audit-controls.md`](16-audit-controls.md) | audit-controls | 10 | 11 | 0 |
| [`17-bff.md`](17-bff.md) | bff | 184 | 10 | 8 |

## Cross-service workflows

Scenarios that span services are repeated (once per participating service)
with the same ID suffix so a single acceptance run can be traced:

- **UC-CS-01** Invoice-to-cash: `invoice` posts → `accounts-receivable` ages →
  `bank-sync` reconciles the payment.
- **UC-CS-02** Procure-to-pay: `accounts-payable` commits → `bank-sync` pays →
  `budget` shows the variance.
- **UC-CS-03** Fixed-asset close: `asset` depreciates → `general-ledger` trial
  balance → `financial-reports` balance sheet.
- **UC-CS-04** Revenue contract: `revenue-recognition` schedules →
  `general-ledger` posts → `financial-reports` income statement.
- **UC-CS-05** Group close: `consolidation` eliminates → `financial-reports`
  group pack.

## Conventions

- Money is always serialized as **strings**; scenarios state the decimal
  precision in force at each service (NUMERIC(19,6) for foundation postings,
  NUMERIC(19,4) for service tables, 2 dp for AR money).
- Tenant isolation (RLS) is assumed in every scenario; one scenario per
  service asserts that cross-tenant data is invisible.
- Scenario names follow `Given / When / Then` and include **worked numbers** so
  an implementer can transcribe them into assertions without inventing data.
