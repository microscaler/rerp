# Consolidation Elimination Engine

Status: scaffold dossier, implementation deferred

Owner service: `consolidation`

Runtime gate: `consolidation` is contract-only until its Rust crates, Helm values, Dockerfile, workspace registration, and generated docs exist.

## Purpose

The consolidation elimination engine evaluates group structures, intercompany balances, currency translation, elimination rules, and group reporting packs.

## Ownership

Owns:

- consolidation groups
- consolidation runs
- elimination rules
- elimination entries
- group reporting packs

Does not own:

- source company GL posting
- operational intercompany transaction creation
- report expression runtime outside group reporting outputs

## Initial Contract Anchors

- `/consolidation-groups`
- `/consolidation-runs`
- `/consolidation-runs/{id}/execute`
- `/elimination-rules`
- `/elimination-entries`
- `/group-reporting-packs`

## Design Questions Before Implementation

- How are group membership and ownership percentages versioned?
- Which eliminations are rule-generated versus accountant-entered?
- How are exchange rates and translation differences sourced?
- How does a consolidation run remain reproducible after source companies change?

## Required BDD Slices

- Execute a consolidation run in dry-run mode.
- Match intercompany AR/AP balances and propose eliminations.
- Generate elimination entries with source-line explanations.
- Produce an immutable group reporting pack.
