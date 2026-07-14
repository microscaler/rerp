# Goal 2: Narrow And Honest Active Runtime

## Objective

Activate only accounting services and operations that contain tested product behavior, while retaining the broader contracts as roadmap material.

## Why This Exists

Generated surface area is not delivered accounting capability. Running hundreds of TODO handlers creates false confidence, increases operational cost, and makes it difficult to distinguish real behavior from examples.

## Broad Outcomes

- General ledger, invoice, and the required API gateway form the initial runtime.
- AR, AP, tax, treasury, reconciliation, reporting, and other domains remain visible in the product roadmap without being auto-deployed as stubs.
- Public runtime operations have explicit implementation ownership.
- Service boundaries follow accounting responsibility and operational need.
- RERP avoids both a premature monolith and runaway microservice abstractions.

## Initial Acceptance Gates

- Tilt and the workspace identify contract-only versus runtime-ready services.
- No deployed endpoint returns generated example accounting data.
- Every public runtime operation has a behavioral test.
- The BFF exposes only authenticated, intentionally supported operations.
- Adding a contract does not silently create a production runtime.
- Runtime activation has a documented checklist and review gate.

## Questions To Thrash Out

- Are invoice and GL separate services for the first slice, or one deployable module with explicit internal boundaries?
- Is a BFF necessary for service consumers, or should RERP expose a dedicated public accounting gateway?
- How should experimental APIs be marked and versioned?
- What evidence is required before a contract-only service becomes runtime-ready?
- How do we prevent generated fallback handlers from reaching production?

## Dependencies

- Goal 1 establishes reliable discovery, generation, and builds.
