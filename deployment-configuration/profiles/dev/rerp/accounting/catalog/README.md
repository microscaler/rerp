# Accounting component catalog

This directory is the Flux-owned holding boundary for Accounting services that
exist in source but have not passed delivery acceptance. Every Helm release is
checked in with `spec.suspend: true`; no image is pulled and no workload is
created.

Suspension may be removed only when the owning service has:

1. a narrowed, authoritative OpenAPI contract;
2. a real user-owned implementation with no generated example responses;
3. identity, authorization, tenant/RLS, and domain acceptance tests;
4. a successful parameterized Tilt image build and clean registry repository;
5. a Flux ImageRepository/ImagePolicy and committed selected tag; and
6. deployment-acceptance coverage for rollout and health.

The active `services/` profile remains a separate Flux component so catalog
changes and suspended releases cannot gate General Ledger or Invoice.

## Audited source state (2026-07-16)

| State | Components |
|---|---|
| Compiles, but generated-example behavior and no service tests | `audit-controls`, `consolidation`, `documents-extraction`, `financial-reports`, `lease-accounting`, `revenue-recognition`, `tax-compliance`, `treasury` |
| Generated contract/implementation drift prevents compilation; no service tests | `accounts-payable`, `accounts-receivable`, `asset`, `bank-sync`, `budget`, `edi`, `bff` |

Compilation alone does not satisfy an activation gate. All fifteen remain
suspended because none has an accepted real-behavior test suite.
