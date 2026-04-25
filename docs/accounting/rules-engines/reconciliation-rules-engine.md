# Reconciliation Rules Engine

Status: design dossier, implementation pending

Owner service: `bank-sync`

Primary BDD slices:

- `docs/ACCOUNTING_BDD_FEATURE_BACKLOG.md` Slice 1A — Reconciliation Suggestions
- `docs/ACCOUNTING_BDD_FEATURE_BACKLOG.md` Slice 1B — Reconcile, Write Off, Exchange Difference, And Unreconcile

## Purpose

The reconciliation rules engine ranks candidate matches for bank transactions and supports accountant-approved reconciliation actions. It should make matching explainable without posting accounting effects until an action is explicitly accepted.

The engine owns:

- reconciliation models
- suggestion generation
- transaction-to-source-record matching decisions
- write-off and exchange-difference adjustment requests
- reconciliation and unreconciliation audit trail

The engine does not own:

- invoice lifecycle
- AR/AP payment lifecycle
- journal-entry posting rules
- bank-provider connection lifecycle
- final general-ledger period lock policy

Those remain with `invoice`, `accounts-receivable`, `accounts-payable`, `general-ledger`, and future provider/import surfaces.

## Current Contract Anchors

Service-local paths:

- `GET /reconciliation-models`
- `POST /reconciliation-models`
- `GET /transactions/{id}/suggestions`
- `POST /transactions/{id}/reconcile`
- `POST /transactions/{id}/unreconcile`
- `POST /transactions/{id}/write-off`
- `POST /transactions/{id}/exchange-difference`

Generated BFF paths:

- `/api/bank-sync/reconciliation-models`
- `/api/bank-sync/transactions/{id}/suggestions`
- `/api/bank-sync/transactions/{id}/reconcile`
- `/api/bank-sync/transactions/{id}/unreconcile`
- `/api/bank-sync/transactions/{id}/write-off`
- `/api/bank-sync/transactions/{id}/exchange-difference`

Key schemas:

- `ReconciliationModel`
- `CreateReconciliationModelRequest`
- `ReconciliationSuggestions`
- `ReconciliationSuggestion`
- `ReconcileTransactionRequest`
- `ReconciliationResult`
- `CreateTransactionWriteOffRequest`
- `CreateExchangeDifferenceRequest`
- `ReconciliationAdjustment`

## Inputs

Required source records:

- bank transaction id, amount, currency, date, reference, description, counterparty, status
- open invoice/payment/journal candidates from source services
- active reconciliation models, ordered by sequence
- company currency and exchange-rate context when currency differs
- GL lock-date status for posting or reversal checks

Optional context:

- counterparty aliases
- payment terms and expected settlement date
- historical match patterns
- amount/date tolerance policy
- write-off accounts and exchange-difference accounts

## Outputs

Read-only outputs:

- ranked suggestions
- confidence values
- rule/model id that produced the suggestion
- explanation text and candidate metadata

Side-effecting outputs:

- reconciliation result
- transaction status transition
- reconciliation audit record
- write-off adjustment request
- exchange-difference adjustment request
- optional journal-entry reference once GL posting is integrated

## Rule Model

Initial model fields:

- `id`
- `name`
- `rule_type`
- `sequence`
- `active`
- `match_tolerance_amount`
- `match_tolerance_days`

Initial rule types:

- `EXACT_MATCH`: exact amount and reference match.
- `AMOUNT_TOLERANCE`: amount within configured tolerance.
- `REFERENCE_MATCH`: invoice/payment/reference text match.
- `COUNTERPARTY_MATCH`: counterparty name/account match.
- `WRITE_OFF`: allowed residual difference.
- `TRANSFER`: internal transfer candidate.

Future rule fields:

- company scope
- journal/account scope
- regex predicates
- partner predicates
- minimum confidence
- auto-apply flag
- dry-run-only flag
- effective date range
- rule version

Rules should be evaluated in deterministic order: active models by sequence, then explicit match score tie breakers.

## Execution Lifecycle

1. **Draft model**: accountant creates a reconciliation model.
2. **Validate model**: service validates required fields and unsupported combinations.
3. **Suggest**: engine evaluates active models against a transaction and source candidates.
4. **Review**: accountant inspects ranked suggestions and explanation.
5. **Accept**: accountant reconciles transaction to a selected source record.
6. **Adjust**: accountant records write-off or exchange difference if needed.
7. **Audit**: service records actor, source records, rule/model version, and side effects.
8. **Unreconcile**: accountant reverses an incorrect reconciliation with a reason.

## Explainability

Every suggestion should include:

- candidate id
- candidate type
- confidence
- model id
- reason
- source fields used
- unmatched or missing fields
- whether accepting it would create side effects

Example explanation:

```text
Matched invoice INV-1001 because transaction reference equals invoice number,
amount equals residual amount, and transaction date is within 2 days of due date.
```

## Auditability

Each accepted action should record:

- actor id
- transaction id
- candidate/source id
- rule/model id and version
- before and after transaction status
- adjustment ids
- journal-entry ids when available
- timestamp
- idempotency key
- unreconcile reason when reversed

Audit records must survive unreconciliation.

## Failure Modes

- Transaction already reconciled.
- Candidate source record is closed, paid, cancelled, or stale.
- Currency differs without exchange-rate context.
- Residual amount exceeds tolerance.
- Required write-off account is missing.
- GL lock date blocks posting.
- Duplicate request uses a conflicting idempotency key.
- Rule references unsupported predicate/action type.

Failures should return typed validation or conflict errors, not silent fallback suggestions.

## OpenAPI Contract Target

Already added:

- `GET /transactions/{id}/suggestions`
- `POST /transactions/{id}/reconcile`
- `POST /transactions/{id}/unreconcile`
- `POST /transactions/{id}/write-off`
- `POST /transactions/{id}/exchange-difference`
- `GET|POST /reconciliation-models`

Needed before full rules engine:

- `GET /reconciliation-models/{id}`
- `PUT /reconciliation-models/{id}`
- `POST /reconciliation-models/{id}/validate`
- `GET|POST /reconciliation-models/{id}/rules`
- `POST /transactions/{id}/edit-reconciliation`
- `POST /transactions/{id}/transfer`
- `POST /transactions/{id}/suggestions/explain` if explanation grows too large for the base suggestions response

## BDD Acceptance

Minimum implementation scenarios:

```gherkin
Feature: Reconciliation suggestions

  Scenario: Exact-reference model ranks an invoice match
    Given a bank transaction has reference "INV-1001" and amount 1200.00
    And an open invoice has number "INV-1001" and residual amount 1200.00
    And an active reconciliation model matches exact reference and amount
    When suggestions are requested for the bank transaction
    Then the invoice is the highest-ranked suggestion
    And the suggestion includes model id, confidence, candidate type, and reason
    And no accounting side effects are created

  Scenario: Accountant accepts a suggested match
    Given a bank transaction has a ranked invoice suggestion
    When the accountant reconciles the transaction to that invoice
    Then the transaction status changes to reconciled
    And the response includes the reconciliation id and status
    And the action is auditable by transaction id, invoice id, actor, and model version

  Scenario: Accountant reverses a reconciliation
    Given a bank transaction is reconciled
    When the accountant unreconciles the transaction with a reason
    Then the transaction returns to unreconciled state
    And the prior reconciliation remains available for audit
```

## Persistence And Events

Likely entities:

- `reconciliation_model`
- `reconciliation_model_rule`
- `reconciliation_suggestion_snapshot`
- `bank_transaction_reconciliation`
- `reconciliation_adjustment`
- `reconciliation_audit_event`

Likely events:

- `bank_sync.reconciliation_model_created`
- `bank_sync.reconciliation_suggestions_requested`
- `bank_sync.transaction_reconciled`
- `bank_sync.transaction_unreconciled`
- `bank_sync.reconciliation_adjustment_created`

Use an idempotency key for side-effecting POSTs.

## Rollout Strategy

1. Implement read-only suggestions in dry-run mode.
2. Add manual reconcile/unreconcile without auto-posting GL entries.
3. Add write-off and exchange-difference adjustments with explicit accountant action.
4. Add model validation and rule management.
5. Add optional auto-apply only after audit, dry-run, and lock-date checks are proven.
