# Lease Accounting Engine

Status: scaffold dossier, implementation deferred

Owner service: `lease-accounting`

Runtime gate: `lease-accounting` is contract-only until its Rust crates, Helm values, Dockerfile, workspace registration, and generated docs exist.

## Purpose

The lease accounting engine classifies leases, calculates lease liabilities and right-of-use assets, generates payment schedules, evaluates modifications, and explains ASC 842 / IFRS 16 style accounting outputs.

## Ownership

Owns:

- lease classification
- payment schedules
- lease liabilities
- right-of-use assets
- lease modifications

Does not own:

- fixed asset depreciation engine outside right-of-use assets
- AP payment execution
- GL posting internals

## Initial Contract Anchors

- `/leases`
- `/lease-payment-schedules`
- `/lease-liabilities`
- `/right-of-use-assets`
- `/lease-modifications`
- `/lease-modifications/{id}/approve`

## Design Questions Before Implementation

- Which lease classification tests are rule-based versus manual?
- How are discount rates sourced and versioned?
- How are modifications recalculated and audited?
- How are right-of-use assets linked to the `asset` service?

## Required BDD Slices

- Classify a lease and explain the classification.
- Generate a lease liability amortization schedule.
- Create right-of-use asset handoff.
- Approve a lease modification and recalculate balances.
