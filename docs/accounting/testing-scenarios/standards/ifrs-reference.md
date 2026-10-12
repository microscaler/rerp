# Accounting Compliance Reference: IFRS (International)

Date: 2026-10-05

Purpose: the international financial reporting standards (IFRS/IAS) each
RERP accounting service must honour, with official IFRS Foundation URLs.
Per-service scenario files cite this file for their standards rows.

Related docs:

- [`../README.md`](../README.md) — scenario suite index and conventions.

## Official reference URLs

| Authority | URL |
|---|---|
| IFRS Standards list (all standards) | <https://www.ifrs.org/issued-standards/list-of-standards/> |
| IFRS Standards Navigator (updates, amendments) | <https://www.ifrs.org/issued-standards/> |
| IASB Conceptual Framework | <https://www.ifrs.org/issued-standards/list-of-standards/conceptual-framework/> |

## Standard catalogue used by this suite

| IFRS/IAS | Title | URL |
|---|---|---|
| IAS 1 | Presentation of Financial Statements | <https://www.ifrs.org/issued-standards/list-of-standards/ias-1-presentation-of-financial-statements/> |
| IAS 2 | Inventories | <https://www.ifrs.org/issued-standards/list-of-standards/ias-2-inventories/> |
| IAS 7 | Statement of Cash Flows | <https://www.ifrs.org/issued-standards/list-of-standards/ias-7-statement-of-cash-flows/> |
| IAS 8 | Accounting Policies, Changes in Accounting Estimates and Errors | <https://www.ifrs.org/issued-standards/list-of-standards/ias-8-accounting-policies-changes-in-accounting-estimates-and-errors/> |
| IAS 12 | Income Taxes | <https://www.ifrs.org/issued-standards/list-of-standards/ias-12-income-taxes/> |
| IAS 16 | Property, Plant and Equipment | <https://www.ifrs.org/issued-standards/list-of-standards/ias-16-property-plant-and-equipment/> |
| IAS 21 | The Effects of Changes in Foreign Exchange Rates | <https://www.ifrs.org/issued-standards/list-of-standards/ias-21-the-effects-of-changes-in-foreign-exchange-rates/> |
| IAS 23 | Borrowing Costs | <https://www.ifrs.org/issued-standards/list-of-standards/ias-23-borrowing-costs/> |
| IAS 36 | Impairment of Assets | <https://www.ifrs.org/issued-standards/list-of-standards/ias-36-impairment-of-assets/> |
| IAS 37 | Provisions, Contingent Liabilities and Contingent Assets | <https://www.ifrs.org/issued-standards/list-of-standards/ias-37-provisions-contingent-liabilities-and-contingent-assets/> |
| IAS 38 | Intangible Assets | <https://www.ifrs.org/issued-standards/list-of-standards/ias-38-intangible-assets/> |
| IFRS 3 | Business Combinations | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-3-business-combinations/> |
| IFRS 7 | Financial Instruments: Disclosures | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-7-financial-instruments-disclosures/> |
| IFRS 9 | Financial Instruments | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-9-financial-instruments/> |
| IFRS 10 | Consolidated Financial Statements | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-10-consolidated-financial-statements/> |
| IFRS 12 | Disclosure of Interests in Other Entities | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-12-disclosure-of-interests-in-other-entities/> |
| IFRS 13 | Fair Value Measurement | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-13-fair-value-measurement/> |
| IFRS 15 | Revenue from Contracts with Customers | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-15-revenue-from-contracts-with-customers/> |
| IFRS 16 | Leases | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-16-leases/> |
| IFRS 18 | Presentation and Disclosure in Financial Statements (forward-looking) | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-18-presentation-and-disclosure-in-financial-statements/> |

Forward-looking note: **IFRS 18** (issued 2024, effective for periods
beginning on or after 1 January 2027) replaces IAS 1; `financial-reports`
scenario files flag the statements it changes.

## Service → IFRS mapping (international)

| Service | Primary IFRS/IAS | Why |
|---|---|---|
| `general-ledger` | IAS 1, IAS 8, IAS 21 | Chart of accounts, journal immutability, period closing, currency books |
| `invoice` | IFRS 15, IFRS 9, IAS 1 | Revenue recognition on posting, AR asset recognition, credit notes as reversal |
| `accounts-payable` | IAS 1, IAS 37, IFRS 9, IAS 21 | Liability recognition, accruals, payment commitments, FX settlement |
| `accounts-receivable` | IFRS 9, IFRS 15, IAS 1 | Receivables at amortised cost, expected credit losses (ECL), aging evidence |
| `asset` | IAS 16, IAS 36, IAS 23, IFRS 13, IAS 21 | Depreciation, impairment, revaluation, borrowing costs |
| `bank-sync` | IAS 7, IAS 21, IAS 1 | Cash balances, bank reconciliation control, FX differences on settlement |
| `budget` | IAS 36 (impairment indicators), IAS 1 (going concern) | Budget-vs-actual as impairment/going-concern evidence; internal control, no direct standard |
| `treasury` | IAS 7, IFRS 9, IFRS 7, IAS 21 | Cash forecasting, liquidity, financial instruments, FX |
| `consolidation` | IFRS 10, IFRS 3, IAS 21, IFRS 12 | Group reporting, eliminations, investment/equity elimination, FX translation |
| `financial-reports` | IAS 1, IAS 7, IAS 8, IFRS 18 (forward) | Balance sheet, income statement, cash flow, trial balance structure |
| `tax-compliance` | IAS 12 | Corporate income tax and deferred tax; indirect taxes (VAT/GST) follow local law, not IFRS |
| `revenue-recognition` | IFRS 15 | Five-step model, over-time vs point-in-time, contract liabilities |
| `lease-accounting` | IFRS 16, IAS 36, IAS 16 | ROU assets, lease liabilities, modifications, impairment of ROU |
| `documents-extraction` | IAS 8, IFRS 15 (evidence), retention law | Source-document evidence for recognition; no direct standard |
| `edi` | IAS 1 / IFRS 15 documentation | Transport/format layer for statutory exchange (e-invoicing follows local law) |
| `audit-controls` | IAS 1 (fair presentation), IAS 8 (errors) | Internal control over financial reporting — COSO/ISA are the primary authorities |
| `bff` | None of its own | Proxy/aggregation layer; inherits each downstream service's standards |
