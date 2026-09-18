# Goal 4: Sesame Tenancy And Database RLS

## Objective

Make tenant isolation a foundational accounting invariant from the public API through PostgreSQL.

## Broad Outcomes

- BRRTRouter validates Sesame JWT/JWKS before dispatch.
- Validated tenant, organization, subject, roles, and service identity become a Lifeguard SessionContext.
- Lifeguard applies transaction-local RLS context through the base executor.
- Tenant-owned rows carry non-null tenant and accounting-entity scope.
- RLS policies cover every tenant-owned accounting table.
- Background workers use explicit Sesame service identities.
- Audit records preserve the initiating service and human subject where applicable.

## Identity Model To Resolve

The first candidate mapping is:

- Sesame tenant: the consuming product realm, initially Hauliage.
- RERP tenant: the isolated accounting customer.
- Accounting entity: the legal entity whose books are held.
- Sesame organization: the authenticated organization acting through the source product.
- Counterparty: a customer, supplier, carrier, or other party represented in accounting documents.

These concepts must not be collapsed merely because the first deployment has one of each.

## Initial Acceptance Gates

- Missing, malformed, expired, or wrong-audience tokens fail closed.
- Caller-supplied tenant or company identifiers cannot override validated context.
- Cross-tenant reads, writes, joins, exports, and document fetches fail.
- Direct application-role database access cannot bypass RLS.
- Worker retries preserve the same tenant and source identity.
- Tenant-safe uniqueness applies to invoice numbers, account codes, external references, and idempotency keys.
- Security tests run against real PostgreSQL policies, not only mocked filters.

## Questions To Thrash Out

- Is RERP tenancy mapped one-to-one with a Sesame organization or through an explicit accounting tenant/entity mapping?
- How are multi-company groups represented without weakening isolation?
- Which service-to-service token flow does Sesame provide?
- How are support, migration, and audit roles granted controlled cross-tenant access?
- What tenant context is legal to place in logs, traces, queues, and object-storage keys?

## Dependencies

- Goal 1 for current BRRTRouter and Lifeguard contracts.
- Goal 3 for deployed JWKS and PostgreSQL integration tests.
