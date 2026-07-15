# RERP Documentation Authority Index

This is the human entry point for determining which RERP concepts are current.
It distinguishes approved product and architecture intent from delivered
runtime behaviour. The LLM wiki is a derived synthesis and is not an authority
registry.

The machine-readable companion is [`authority.json`](./authority.json). CI
validates its targets, statuses, ADR coverage, and supersession relationships.
The complete lifecycle policy is
[`DOCUMENTATION_GOVERNANCE.md`](./DOCUMENTATION_GOVERNANCE.md).

## Two kinds of truth

### Normative truth: what RERP should do

Use this order when deciding the intended product or architecture:

1. Accepted Architecture Decision Records.
2. Active operating models and approved PRDs.
3. Active suite and service designs.
4. Authoritative OpenAPI contracts.
5. Acceptance criteria and policy tests.

### Delivered truth: what RERP does now

Use this order when diagnosing current behaviour:

1. Deployed runtime behaviour and persisted schema.
2. User-owned implementation code and configuration.
3. OpenAPI inputs and generated contracts.
4. Tests and verification records.
5. Dated implementation-status reports.

When these views disagree, record explicit implementation drift. Delivered
code does not silently overturn an accepted decision, and an approved design
does not prove that the runtime already implements it.

## Current authority map

| Scope | Current authority | Status | Notes |
|---|---|---|---|
| Repository structure and contribution | [`CONTRIBUTING.md`](../CONTRIBUTING.md) | Active | Canonical suite, service, generation, migration, and ownership contract |
| Accounting runtime boundary | [ADR 001](./adrs/001-accounting-runtime-boundary.md) | Accepted | Invoice-to-GL atomicity and first runtime boundary |
| Generated-document ownership | [ADR 002](./adrs/002-document-generation-ownership.md) | Accepted | Documents owns rendering; source suites own business facts |
| Hauliage accounting dogfood | [Dogfood roadmap](./roadmap/hauliage-accounting-dogfood/README.md) | Proposed working plan | Execution overlay; does not settle accounting policy |
| Hauliage commercial behaviour | [Hauliage Commercial Mode of Operation](https://github.com/microscaler/hauliage/blob/main/docs/prd-commercial-mode-of-operation.md) | Working draft | External product authority at `microscaler/hauliage:docs/prd-commercial-mode-of-operation.md`; RERP must record the reviewed revision before deriving accounting requirements |
| Accounting public API | `openapi/accounting/*/openapi.yaml` | Active contract inputs | Individual service specs are authoritative; `openapi_bff.yaml` is their suite aggregation |
| Documents Render public API | [`openapi/documents/render/openapi.yaml`](../openapi/documents/render/openapi.yaml) | Active contract input | Must conform to ADR 002 and the approved rendition PRD |
| Accounting delivery readiness | [Service readiness plan](./roadmap/hauliage-accounting-dogfood/service-readiness-plan/README.md) | Proposed working plan | Checkable delivery surface, not proof of implementation |

## Architecture decisions

The authoritative ADR register and next identifier are maintained in
[`adrs/README.md`](./adrs/README.md). Accepted ADRs are immutable except for
metadata and clarification that does not change the decision. A changed
decision requires a new ADR that supersedes the old one.

## Document classes

| Class | Purpose | Authority |
|---|---|---|
| ADR | Durable accepted decision and consequences | Normative when accepted |
| Mode of Operation | Current end-to-end commercial or operational behaviour | Normative when approved |
| PRD | Requirements and acceptance criteria | Normative when approved |
| Design | Technical realization of approved requirements and decisions | Normative while active |
| Roadmap/plan | Sequencing and delivery intent | Working authority only |
| Analysis/audit | Dated evidence and alternatives | Informative; never silently normative |
| Implementation status | Dated delivered-state snapshot | Evidence only |
| LLM wiki | Reconciled navigation and drift synthesis | Derived, never normative |

## Historical documents

Do not infer authority from filenames such as `COMPLETE`, modification time, or
search rank. Legacy documents that have not yet adopted governance metadata are
informative until they are listed here or in `authority.json`.

Known historical material remains in place to preserve links and reasoning.
When encountered, add a visible `SUPERSEDED` or `HISTORICAL_SNAPSHOT` banner
with a link to the current authority rather than deleting or silently moving it.

The first reviewed historical tranche is indexed in
[`history/README.md`](./history/README.md). It contains superseded bootstrap,
generation, entity-layout, implementation-status, audit, and execution-order
documents whose current replacements are known.

## Updating current authority

A change that introduces, accepts, implements, or supersedes a concept must:

1. Update the authoritative document and its lifecycle metadata.
2. Add or update an ADR when an accepted decision changes.
3. Update this index and `authority.json` when the current authority changes.
4. Mark the replaced document explicitly and reciprocally.
5. Reconcile the relevant LLM wiki topic and append its log.
6. Run `python tooling/scripts/check_doc_governance.py`.
