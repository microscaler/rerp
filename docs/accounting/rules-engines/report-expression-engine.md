# Report Expression Engine

Status: design dossier, implementation pending

Owner service: `financial-reports`

Primary BDD slice:

- `docs/ACCOUNTING_BDD_FEATURE_BACKLOG.md` Slice 1C — Report Definitions And Drill-Down

## Purpose

The report expression engine evaluates configurable financial report definitions into auditable report executions. It should produce report lines, cells, drill-down source lines, and exports from accounting source data without hiding formula behavior.

The engine owns:

- report definitions
- report lines
- expressions
- execution options
- report cell outputs
- source-line drill-down
- report export requests

The engine does not own:

- journal-entry posting
- AR/AP collection or payment state
- tax filing submission
- consolidation eliminations
- source-ledger mutation

Those remain with `general-ledger`, `accounts-receivable`, `accounts-payable`, `tax-compliance`, and future consolidation services.

## Current Contract Anchors

Service-local paths:

- `GET /report-definitions`
- `POST /report-definitions`
- `GET /report-definitions/{id}/lines`
- `GET /report-cells/{id}/drill-down`
- `POST /report-exports`
- `GET /statutory-report-packs`

Generated BFF paths:

- `/api/financial-reports/report-definitions`
- `/api/financial-reports/report-definitions/{id}/lines`
- `/api/financial-reports/report-cells/{id}/drill-down`
- `/api/financial-reports/report-exports`
- `/api/financial-reports/statutory-report-packs`

Key schemas:

- `ReportDefinition`
- `CreateReportDefinitionRequest`
- `ReportDefinitionLine`
- `PaginatedReportDefinitionLines`
- `ReportCellDrillDown`
- `CreateReportExportRequest`
- `ReportExport`
- `StatutoryReportPack`

## Inputs

Required source records:

- report definition id
- report lines with expression strings
- company id
- fiscal period or date range
- company currency and reporting currency
- GL account balances, journal entries, journal items

Optional source records:

- AR aging and partner balances
- AP aging and supplier balances
- tax return working balances
- budget values
- prior-period report execution
- analytic dimensions

Execution options:

- date range
- comparison period
- currency
- unfold level
- include draft or posted-only data
- partner/account filters
- export format

## Outputs

Read-only outputs:

- report execution status
- report lines
- report cells
- expression results
- source-line drill-down

Artifact outputs:

- PDF export
- XLSX export
- CSV/JSON export
- statutory report pack artifact

Audit outputs:

- report definition version
- execution options
- source data window
- source-line references
- export artifact uri

## Expression Model

Initial expression shape:

- expression string stored on `ReportDefinitionLine`
- expression references ledger source data by stable identifiers
- expression evaluation is read-only
- output is numeric or text, depending on report line type

Initial expression categories:

- `ACCOUNT_BALANCE`: account or account-group balances.
- `JOURNAL_SUM`: debit, credit, or balance over journal items.
- `FORMULA`: arithmetic over other report lines.
- `CONSTANT`: static values or labels.
- `SOURCE_LINK`: source-line drill-down pointer.

Future expression fields:

- expression language/version
- variable bindings
- validation diagnostics
- dependency graph
- cache key
- localization extension key
- analytic dimension filters

Expressions must be deterministic for the same definition version, options, and source data window.

## Execution Lifecycle

1. **Draft definition**: accountant creates report definition.
2. **Add lines**: lines define label, sequence, parent, expression, and display behavior.
3. **Validate definition**: service validates references, expression syntax, and dependency graph.
4. **Execute report**: service evaluates the definition against source data and options.
5. **Inspect drill-down**: accountant drills from cells to source lines.
6. **Export**: accountant requests PDF/XLSX/CSV/JSON export.
7. **Archive**: immutable execution/export artifacts remain reproducible by definition version and options.

## Explainability

Every report cell should be explainable by:

- report definition id and version
- report line id
- expression text
- evaluated inputs
- source record ids
- source service
- execution options
- result value

Drill-down should show source lines with:

- source id
- source type
- amount
- description
- account/partner when available
- posting date when available

## Auditability

Each report execution should record:

- actor id or scheduled job id
- definition id and version
- execution options
- source data cutoff
- source services queried
- generated report execution id
- generated export ids
- checksum or artifact uri for exports

Exports should be immutable. If a report definition changes, a new version should be used for future executions rather than mutating the meaning of historical reports.

## Failure Modes

- Invalid expression syntax.
- Unknown account, line, variable, or source reference.
- Circular expression dependency.
- Missing source data from GL/AR/AP.
- Currency conversion context missing.
- Execution options conflict with report type.
- Export requested before execution completes.
- Source data changed after execution but before export.

Failures should return typed validation or conflict errors with expression line references when possible.

## OpenAPI Contract Target

Already added:

- `GET|POST /report-definitions`
- `GET /report-definitions/{id}/lines`
- `GET /report-cells/{id}/drill-down`
- `POST /report-exports`
- `GET /statutory-report-packs`

Needed before full expression engine:

- `GET /report-definitions/{id}`
- `PUT /report-definitions/{id}`
- `POST /report-definitions/{id}/validate`
- `GET|POST /report-definitions/{id}/expressions`
- `POST /report-definitions/{id}/execute`
- `GET /report-executions/{id}/lines`
- `GET /report-executions/{id}/cells/{cell_id}/source-lines`
- `POST /report-schedules`
- `POST /report-schedules/{id}/run-now`

## BDD Acceptance

Minimum implementation scenarios:

```gherkin
Feature: Configurable report definitions

  Scenario: Accountant creates a reusable report definition
    Given the accountant has permission to configure reports
    When a report definition is created with name, type, and company scope
    Then the definition is returned with an id and active status
    And it appears in the report definition list

  Scenario: Report line drill-down returns source accounting records
    Given a completed report execution has a cell balance
    When the accountant drills into that report cell
    Then the response lists source lines with source id, source type, amount, and description
    And each source line can be traced to a ledger, AR, AP, or tax source record

  Scenario: Accountant exports a completed report
    Given a report execution has completed
    When the accountant requests a PDF export
    Then the export request is accepted
    And the export records report execution id, status, format, and artifact uri when ready
```

## Persistence And Events

Likely entities:

- `report_definition`
- `report_definition_version`
- `report_definition_line`
- `report_expression`
- `report_execution`
- `report_execution_line`
- `report_execution_cell`
- `report_source_line`
- `report_export`
- `statutory_report_pack`

Likely events:

- `financial_reports.report_definition_created`
- `financial_reports.report_definition_validated`
- `financial_reports.report_executed`
- `financial_reports.report_drilldown_requested`
- `financial_reports.report_export_requested`
- `financial_reports.report_export_ready`

Use definition version and execution options as part of report execution reproducibility.

## Rollout Strategy

1. Implement report definition CRUD and line listing using typed contracts.
2. Add validation for report definition and line references.
3. Add read-only execution for simple account-balance expressions.
4. Add drill-down from report cells to GL source lines.
5. Add export request lifecycle.
6. Add richer formula expressions, partner ledgers, AR/AP aging, tax reports, and schedules.
