# ADR 003: Workplace booking is a separate suite with Spaces and Bookings services

- **Status**: PROPOSED
- **Date**: 2026-09-18
- **Decision owners**: RERP maintainers
- **Group**: Suite and service boundaries
- **Authority**: Normative
- **Scope**: workplace.suite-boundary
- **Last reviewed**: 2026-09-18
- **Supersedes**: None
- **Superseded by**: None

## Context

RERP needs a first customer-facing capability that is small enough to deliver
end to end, is driven by real procurement requirements rather than invented
ones, and exercises identity (Sesame IDAM), RBAC, Documents, and Analytics
together. Desk and room booking for shared offices meets those conditions.

A concrete public requirement exists: the City of Chemnitz tender
"Raumbuchungsprogramm für Desksharing" (TED notice 595241-2026, published
2026-08-28, deadline 2026-09-30). It asks for configurable bookable resources
(workstations, meeting rooms, possibly vehicles), space-utilisation analysis,
roughly 1,800 resource licences shared by about 5,000 users, hosting, support,
training, and an annual development budget. The full specification documents
had not been reviewed when this ADR was drafted; see the PRD for what is known
and what is assumed.

Desk booking is spatial. Users choose a desk on a floor plan, and
administrators maintain plans made of walls, rooms, zones, and resources. No
existing RERP service owns that model:

- `appointments/core` models customer appointments against abstract resources
  and time slots. It has no spatial model, check-in, or occupancy policy.
- `accounting/asset` owns fixed assets for depreciation and accounting. A desk
  is not an accounting asset, and coupling bookings to the ledger would violate
  the suite-selective installation rule.

## Decision

Create a new `workplace` suite with two services:

1. `workplace/spaces` owns the physical model: sites, buildings, floors with a
   calibrated background plan, floor geometry as a graph of corners and walls
   with hosted openings, rooms derived from closed wall loops, zones with access
   rules, and bookable resources with geometry.
2. `workplace/bookings` owns reservations of those resources: bookings,
   recurrence, group bookings, check-in and no-show release, booking policies,
   administrative blocks, availability for map rendering, and daily utilisation
   facts.

Each service owns its own tables. Bookings refers to resources by identifier and
reads resource and access data from Spaces through its API, never through its
tables. Identity and group or role membership come from the Auth suite. Plan
images are stored in the Documents suite and referenced by document identifier.
Utilisation facts are exposed for the Analytics suite to consume; Workplace does
not depend on Analytics. Bookings publishes lifecycle events for the Notifications
suite to consume; it does not send email directly.

QR and NFC check-in credentials are proof-of-presence secrets. Spaces owns and
rotates them through a facility-administrator-only endpoint. Ordinary resource
list and detail responses expose only whether check-in is supported, never the
credential itself.

Floor geometry is stored in real units (millimetres) in each floor's coordinate
system, not as image-relative percentages, so plans survive a replaced
background image and support true-scale bulk placement.

The editor design (wall graph, derived rooms, hosted openings, snapping) is
implemented clean-room from published concepts. No code from third-party floor
planners is copied.

## Assumptions

- The Chemnitz specification does not require capabilities that would change
  this boundary (for example, integration with a building-automation system as
  a system of record).
- Auth can resolve a caller's groups and roles for access-rule evaluation
  within a request.
- Documents can store images and PDF pages that the UI renders as plan
  backgrounds.
- Notifications can consume booking lifecycle events without a synchronous
  dependency from Workplace.

## Alternatives considered

### Extend `appointments/core`

Adding floors, geometry, and check-in to Appointments would make one service
model both customer appointments and internal space use. The concepts share
only "a resource is reserved for a time window". The spatial model, access
rules, and occupancy analytics would dominate the service and distort its
customer-appointment contract. Rejected.

### One `workplace` service

Simpler to deploy, but editing geometry and serving high-volume availability
queries have different change rates, scaling profiles, and permissions. Two
services keep plan editing isolated from booking traffic. A single service
could be revisited if the second service adds operational cost without benefit
in practice.

### Model desks as `accounting/asset` records

Rejected: wrong semantics and a forbidden cross-suite table dependency.

## Consequences

### Positive

- A self-contained, demonstrable product slice that exercises the platform
  services RERP needs to prove.
- Contracts are grounded in a real public requirement and can be checked
  against it.
- Room polygons and resource geometry serve booking, heatmaps, and reporting
  from one source.

### Costs and risks

- A new suite adds port assignments, Helm descriptors, Tilt wiring, and
  deployment profiles.
- Bookings depends on Spaces availability at booking time; a Spaces outage
  blocks new bookings.
- The floor-plan editor is significant UI work and is the component
  evaluators judge first.
- Public-sector buyers may require the delivered application to be open
  source. The licence of the Workplace application relative to the RERP
  platform licence is an open business decision recorded in the PRD.

## Implementation and verification

- Contracts: `openapi/workplace/spaces/openapi.yaml` and
  `openapi/workplace/bookings/openapi.yaml` (version 0.1.0, proposed).
- Scaffolding (ports, `gen/` and `impl/` crates, Helm, Tilt, deployment
  profiles) follows `CONTRIBUTING.md` after this ADR is accepted.
- Acceptance: the PRD acceptance criteria, verified by service tests in each
  `impl/` crate and an end-to-end booking scenario on a published floor.

## Related requirements

- [Workplace booking PRD](../workplace/WORKPLACE_BOOKING_PRD.md) (DRAFT)

## Related decisions and artifacts

- [ADR 002](./002-document-generation-ownership.md): Documents ownership of
  stored renditions.
- External requirement source: TED notice 595241-2026 (Stadt Chemnitz),
  reviewed 2026-09-18 from the public notice only.

## Notes

None.
