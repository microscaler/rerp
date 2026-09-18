# Hauliage Accounting Dog-Food Roadmap

- **Status**: Proposed execution overlay
- **Created**: 2026-07-14
- **First consumer**: Hauliage
- **Product destination**: World-class, open-source, API-first ERP and accounting platform

## Purpose

RERP is the accounting system. Hauliage is its first real SaaS consumer and must use RERP through the same public APIs that any external product can use.

This roadmap is the immediate execution overlay for the broader accounting roadmap. It does not reduce the long-term scope of RERP. RERP is still intended to deliver full ledger, tax, treasury, reconciliation, reporting, controls, and adjacent ERP capabilities. The Hauliage tranche establishes the trustworthy foundation on which those capabilities can be built.

## Product Thesis

RERP should let a SaaS product integrate its complete accounting lifecycle without embedding accounting rules into that product.

Hauliage should own freight execution facts such as quotes, parties, ePOD, delivery, disputes, and settlement events. RERP should own accounting documents, numbering, tax treatment, posting, journals, audit history, and financial state.

The proof is not that RERP can special-case Hauliage. The proof is that Hauliage can use a reusable, well-documented, versioned RERP API as an ordinary tenant.

## Product Principles

1. **API first, including first-party use**
   - OpenAPI is the public product contract.
   - RERP user interfaces, Hauliage, SDKs, and third-party consumers use the same APIs.
   - No privileged private accounting path may become the real implementation.

2. **Open-source core without SaaS-only accounting**
   - Core accounting, migrations, controls, and APIs remain usable in a self-hosted deployment.
   - Hosted RERP may add operations and managed-service value, but not hide the accounting engine behind proprietary internal APIs.

3. **Domain-neutral accounting contracts**
   - RERP models accounting concepts, source references, and extensible dimensions.
   - Hauliage-specific freight types stay in Hauliage.
   - A source system can provide commercial facts without dictating RERP internals.

4. **Tenant-safe by construction**
   - Sesame identity establishes the tenant, organization, subject, and service principal.
   - BRRTRouter validates identity before application code sees it.
   - Lifeguard applies transaction-scoped RLS context.
   - Caller-supplied identifiers cannot override authenticated tenant scope.

5. **Accounting correctness before breadth**
   - Posted documents are immutable.
   - Entries balance.
   - Money uses decimal types and explicit ISO currencies.
   - Period locks, audit trails, idempotency, and reversal semantics are foundational.

6. **Dog-food without product contamination**
   - Hauliage drives realistic requirements and acceptance tests.
   - Every Hauliage requirement must be tested for whether it is reusable, configurable, or genuinely domain-specific.
   - Domain-specific behavior remains behind Hauliage's integration boundary.

7. **Incremental runtime, broad roadmap**
   - Contract research and future designs can remain broad.
   - Only implemented, tested services are activated in the build and runtime.
   - New runtime boundaries are justified by ownership and scaling needs, not generated surface area.

## Cross-Cutting Decision Gate

Before Goals 5 and 6 can be finalized, Hauliage and RERP need an accounting-policy ADR covering:

- whether the marketplace is principal or agent;
- who invoices whom;
- whether carrier documents are supplier invoices or self-billed documents;
- which charges are revenue, pass-throughs, liabilities, insurance, or tax;
- the initial jurisdiction, tax point, and rounding rules;
- when ePOD, delivery audit, dispute resolution, and settlement cause accounting events;
- how adjustments create credit/debit notes rather than mutate posted documents;
- the authoritative commission and insurance terms.

The gate is accepted when one representative haul has approved source documents and balanced example journals for every relevant lifecycle event.

## The Seven Goals

| Goal | Outcome | Detail |
|---|---|---|
| 1 | A reproducible RERP development baseline | [Goal 1](./goal-01-development-baseline/README.md) |
| 2 | A narrow, honest active accounting runtime | [Goal 2](./goal-02-active-runtime/README.md) |
| 3 | Repeatable shared-cluster delivery | [Goal 3](./goal-03-shared-cluster-delivery/README.md) |
| 4 | Sesame-backed tenancy and database RLS | [Goal 4](./goal-04-tenancy-and-rls/README.md) |
| 5 | A correct minimum accounting model | [Goal 5](./goal-05-accounting-foundation/README.md) |
| 6 | A public invoice-to-GL vertical slice | [Goal 6](./goal-06-invoice-to-gl/README.md) |
| 7 | Hauliage integrated as the first ordinary consumer | [Goal 7](./goal-07-hauliage-integration/README.md) |

## Working Delivery Plan

The seven goals provide the strategic dependency shape. Day-to-day delivery of
the Accounting and Documents services required by Hauliage is tracked in the
[Hauliage Accounting Service Readiness Work-Through Plan](./service-readiness-plan/README.md).

That plan is the checkable execution surface for structural reconciliation,
General Ledger, Invoice, AR, AP, banking/connectors, reporting, Documents
Render, the Accounting BFF, and generated-client dogfood acceptance.

## Dependency Shape

Goals 1 through 4 establish development and trust foundations. Goal 5 establishes accounting invariants. Goal 6 proves RERP's public accounting API. Goal 7 proves that an external SaaS system can use that API reliably.

Some work can overlap:

- Goal 3 can progress once Goal 1 establishes the toolchain shape.
- Goal 4 can start contract and threat-model work before the cluster is complete.
- The accounting-policy ADR can progress alongside Goals 1 through 4.
- Goal 5 must not freeze posting rules before the policy ADR is accepted.
- Goal 7 can define its consumer contract early, but cannot replace Goal 6 with Hauliage-specific endpoints.

## First Dog-Food Outcome

For a qualifying Hauliage delivery:

1. Hauliage records the freight transition and durable outbox event.
2. A typed client submits an idempotent accounting instruction to RERP.
3. RERP derives tenancy from validated Sesame identity.
4. RERP creates an immutable invoice and balanced journal atomically.
5. RERP renders and stores the accounting document.
6. Hauliage retrieves the RERP number, totals, status, and document.
7. Retrying any failed step cannot duplicate the accounting result.

This outcome does not pretend that quote acceptance is payment, or that a boolean escrow flag proves cash movement. Cash, settlement, and reconciliation entries require authoritative provider/bank events.

## Definition Of Ready For Broader Accounting Work

The broader roadmap may resume active runtime expansion when:

- the RERP workspace, generation, tests, and migrations are reproducible;
- the shared-cluster deployment is healthy and observable;
- authentication and RLS fail closed under cross-tenant tests;
- no active accounting endpoint returns generated example data;
- Hauliage has issued at least one invoice through the public RERP API;
- the invoice has a balanced, inspectable journal and immutable document;
- retries, failures, and tenant isolation are proven end to end;
- the resulting contracts contain no freight-only assumptions.

## Out Of Scope For This First Pass

This document does not settle the detailed schemas, service boundaries, accounting policy, tax jurisdiction, or implementation estimates. Those decisions belong in the goal documents and ADRs as the goals are thrashed out.

It also does not remove the broader accounting specifications. It changes execution order: prove the accounting core with a real consumer before activating the rest of the product surface.

## Related Documents

- [Hauliage Accounting service readiness work-through plan](./service-readiness-plan/README.md)
- [Document generation and rendition PRD](../../../openapi/documents/DOCUMENTS_ANALYSIS/PRDs/PRD-008-Document-Generation-and-Rendition.md)
- [Document generation ownership ADR](../../adrs/002-document-generation-ownership.md)
- [Accounting implementation roadmap](../../../openapi/accounting/design/08-implementation-roadmap.md)
- [Historical Accounting suite build plan](../../history/plans/ACCOUNTING_BUILD_PLAN.md)
- [Accounting BDD feature backlog](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md)
- [Enterprise ERP gap analysis](../../ACCOUNTING_ENTERPRISE_ERP_GAP_ANALYSIS.md)
- [Hauliage reference operating model](../../llmwiki/topics/hauliage-reference-operating-model.md)
