# Workplace

> **Status: PROPOSED** — see [ADR 003](../../docs/adrs/003-workplace-booking-suite-boundary.md)
> and the [Workplace booking PRD](../../docs/workplace/WORKPLACE_BOOKING_PRD.md) (DRAFT).
> No service crates, ports, or deployment profiles exist yet.

## Overview

Desk, meeting-room, parking, and pool-vehicle booking for shared offices,
driven by floor plans.

## Services

### Spaces
- **Path**: `workplace/spaces/`
- **Description**: Sites, buildings, floors with calibrated background plans,
  wall-graph floor geometry with derived rooms, zones with access rules, and
  bookable resources.
- **Documentation**: [Spaces README](./spaces/README.md)
- **API Spec**: [Spaces OpenAPI](./spaces/openapi.yaml)

### Bookings
- **Path**: `workplace/bookings/`
- **Description**: Bookings, recurrence, group bookings, check-in and no-show
  release, policies, administrative blocks, availability, and utilisation
  facts.
- **Documentation**: [Bookings README](./bookings/README.md)
- **API Spec**: [Bookings OpenAPI](./bookings/openapi.yaml)

## Integration Patterns

- Identity, groups, and roles: Auth suite (Sesame IDAM, RBAC), by API.
- Plan images: Documents suite, referenced by document ID.
- Utilisation reporting: Analytics consumes `GET /v1/utilisation`; Workplace
  does not depend on Analytics.
- Bookings reads resources and access rules from Spaces by API, never by table.
