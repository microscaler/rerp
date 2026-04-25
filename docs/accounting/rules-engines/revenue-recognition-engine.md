# Revenue Recognition Engine

Status: scaffold dossier, implementation deferred

Owner service: `revenue-recognition`

Runtime gate: `revenue-recognition` is contract-only until its Rust crates, Helm values, Dockerfile, workspace registration, and generated docs exist.

## Purpose

The revenue recognition engine creates recognition schedules from invoices or contracts, calculates period recognition, posts or requests journal entries, and explains deferred revenue or expense balances.

## Ownership

Owns:

- recognition rules
- recognition schedules
- deferred revenue and deferred expense records
- recognition runs
- recognition journal-entry handoff

Does not own:

- invoice lifecycle
- GL posting internals
- lease-accounting schedules
- cash collection

## Initial Contract Anchors

- `/recognition-rules`
- `/recognition-schedules`
- `/deferred-revenues`
- `/deferred-expenses`
- `/recognition-runs`
- `/recognition-runs/{id}/post`

## Design Questions Before Implementation

- Which recognition methods are core versus extension-specific?
- How are invoice lines mapped to recognition rules?
- How are schedule modifications versioned?
- What is the reversal behavior for posted recognition runs?

## Required BDD Slices

- Create a recognition schedule from an invoice handoff.
- Simulate a recognition run for a fiscal period.
- Post a recognition run with source schedule references.
- Explain deferred revenue balance by schedule line and rule version.
