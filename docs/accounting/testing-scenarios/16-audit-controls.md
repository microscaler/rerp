# Audit Controls — Use-Case and Testing Scenarios

Date: 2026-10-05

- **Status**: PROPOSED
- **Owner**: accounting maintainers
- **Scope**: microservices/accounting/audit-controls
- **Last reviewed**: 2026-10-05

Purpose: acceptance scenarios for approval policies, segregation-of-duties
rules, signature requests, immutable audit events, and control exceptions.

Related docs:

- [`standards/ifrs-reference.md`](standards/ifrs-reference.md) — international standards (IFRS/IAS).
- [`../rules-engines/audit-controls-policy-engine.md`](../rules-engines/audit-controls-policy-engine.md)
- [`../../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md)

## Standards reference

Audit controls implement internal control over financial reporting; the
closest authorities are control frameworks, not IFRS.

### International control frameworks

| Authority | Relevance | URL |
|---|---|---|
| COSO — Internal Control — Integrated Framework | Control environment, risk assessment, control activities | <https://www.coso.org/> |
| ISA 315 — Identifying and Assessing the Risks of Material Misstatement | Control design evidence auditors rely on | <https://www.iaasb.org/> (IAASB standards) |

### IFRS (international)

| Standard | Relevance | URL |
|---|---|---|
| IAS 1 — Presentation of Financial Statements | Fair presentation supported by controls | <https://www.ifrs.org/issued-standards/list-of-standards/ias-1-presentation-of-financial-statements/> |
| IAS 8 — Accounting Policies, Changes in Estimates and Errors | Material-error detection/correction | <https://www.ifrs.org/issued-standards/list-of-standards/ias-8-accounting-policies-changes-in-accounting-estimates-and-errors/> |

## Endpoint inventory (10 operations)

Approval policies (list/create); segregation rules (list/create); signature
requests (list/create/sign); audit events (list); control exceptions
(list/create).

## Use-case scenarios

### UC-AUD-01 — Approval policy with minimum approvals

Given `POST /approval-policies {service_name: "accounts-payable", action_name: "approve_payment_batch", minimum_approvals: 2}`.
Then the policy is ACTIVE; when a payment batch is submitted for approval,
two distinct approvers must approve before the batch can be exported —
one approval leaves the action blocked with the missing-approval reason.

### UC-AUD-02 — Segregation of duties

Given `POST /segregation-rules {service_name: "accounts-payable", initiator_role: "ap-clerk", approver_role: "ap-manager"}`.
When the same actor who initiated a vendor invoice attempts to approve it.
Then blocked with the SoD violation reference; an actor whose roles do not
conflict can approve; creating a rule whose roles are identical is rejected.

### UC-AUD-03 — Cross-service policy evaluation

Given policies on `general-ledger` (action `close_period`) and `invoice`
(action `post_credit_note`).
Then each service's sensitive action consults this service at execution
time; a policy deactivated mid-flight applies from the next evaluation
(no retroactive effect on already-approved actions).

### UC-AUD-04 — Signature request lifecycle

Given `POST /signature-requests {service_name: "financial-reports", entity_id: <report-id>, requested_by}`.
Then REQUESTED → SIGNED (or REJECTED/EXPIRED/CANCELLED); signing requires
`signed_by` + `declaration: true`; a request already SIGNED/REJECTED cannot
be signed again; expired requests cannot be signed.

### UC-AUD-05 — High-risk actions require signature

Given a policy requiring signature for `accounts-payable:pay_over_10000`.
When a payment of 15,000.00 is attempted without a SIGNED request.
Then the action is blocked until the request is signed; a 9,000.00 payment
passes without a signature (threshold boundary).

### UC-AUD-06 — Immutable audit event stream

Given events emitted by `invoice` (INVOICE_POSTED) and `general-ledger`
(period close).
When `GET /audit-events?service_name&entity_id`.
Then events are returned in `occurred_at` order with `actor_id` and
`action`; the stream is append-only — no update/delete endpoints exist, and
any tampering attempt is a 404/405.

### UC-AUD-07 — Control exception workflow

Given `POST /control-exceptions {service_name, entity_id, reason: "urgent payment during approver leave"}`.
Then OPEN → WAIVED or REMEDIATED or REJECTED → CLOSED; a WAIVED exception
records the waiving actor and rationale (audit evidence); operating a
control action against an exception without resolution is flagged.

### UC-AUD-08 — Policy/rule activation

Given an inactive approval policy or segregation rule.
Then it never blocks anything; activating it applies from the next
evaluation; historical evaluations keep the rule version that decided them.

### UC-AUD-09 — Boundary validation

Given page 0, limit 101, negative `minimum_approvals`, empty
service_name/action_name.
Then each is rejected 400; defaults page=1 limit=20.

### UC-AUD-10 — Tenant isolation

Given policies/events of tenant A and tenant B.
Then tenant B sees only its own rows; cross-tenant entity ids resolve to
404; audit events of tenant A are never visible to tenant B's reviewers.

### UC-AUD-11 — Contract-only status guard

Given the service is contract-only today (stub controllers; `sign` returns
an empty status; no models/migrations).
Then scenarios fail loudly until implemented.

## Test coverage matrix

All scenarios: **not yet implemented** — no models, no migrations, no
tests; all 11 controllers are stubs.

| Scenario | Existing test | Status |
|---|---|---|
| UC-AUD-01..11 | none | not yet implemented |

## Known drift / gaps

- Contract-only; `gen` handler types model `ControlException.status` and
  `SignatureRequest.status` as `serde_json::Value` instead of the OpenAPI
  enums — shape mismatch to fix during implementation.
- The policy engine dossier defines ownership (approval policies, SoD,
  signatures, audit-event query surface, exceptions) and required BDD
  slices — UC-AUD-01/02/05/06/07 are their granular form.
- The audit event store itself lives in foundation
  (`accounting_audit_events`); this service owns the **query and policy**
  surface, not event storage.
