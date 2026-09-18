# Audit Controls Policy Engine

Status: scaffold dossier, implementation deferred

Owner service: `audit-controls`

Runtime gate: `audit-controls` is contract-only until its Rust crates, Helm values, Dockerfile, workspace registration, and generated docs exist.

## Purpose

The audit controls policy engine evaluates approval policies, segregation-of-duties rules, signature requirements, audit-event queries, and control exceptions across accounting workflows.

## Ownership

Owns:

- approval policies
- segregation rules
- signature requests
- cross-service audit event query surfaces
- control exceptions

Does not own:

- the business workflow being approved
- service-local validation rules
- authentication or identity lifecycle
- immutable event storage implementation if provided by a shared platform component

## Initial Contract Anchors

- `/approval-policies`
- `/segregation-rules`
- `/signature-requests`
- `/signature-requests/{id}/sign`
- `/audit-events`
- `/control-exceptions`

## Design Questions Before Implementation

- Which controls remain service-local and which must be centralized?
- How are identity roles and delegation sourced?
- How are policy decisions made explainable to auditors?
- What is the retention and immutability requirement for audit events?

## Required BDD Slices

- Block approval when initiator and approver violate segregation rules.
- Require electronic signature for high-risk accounting actions.
- Create a control exception with waiver and remediation workflow.
- Query audit events across services by entity id and action.
