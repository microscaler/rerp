# Workplace Spaces

> **Status: PROPOSED** contract, version 0.1.0. See
> [ADR 003](../../../docs/adrs/003-workplace-booking-suite-boundary.md).

## Responsibility

Owns the physical model of bookable space:

- **Sites, buildings, floors** with IANA time zones and ordinal levels.
- **Background plans**: an image or PDF page stored in Documents, calibrated by
  two image points and a real distance.
- **Floor geometry** as one versioned document: corners, walls between corners,
  and openings hosted on walls. Saved with optimistic concurrency
  (`base_revision`; `409` if stale).
- **Rooms** derived from closed wall loops after each save. Rooms can be named
  and classified but not drawn.
- **Zones**: polygons with group/role access rules.
- **Resources**: desks, meeting rooms, parking spaces, vehicles, and lockers
  with geometry, attributes, assignment mode, and a QR check-in code.

Coordinates are millimetres in each floor's own coordinate system.

## Design notes

- Moving a corner moves every wall that shares it; this is what makes tracing
  feel like CAD rather than drawing.
- A plan save never deletes resources. Resources left outside every room are
  returned as `PlanWarning` entries.
- `POST /v1/floors/{id}/resource-batches` places a grid of desks in one call,
  which matters at the scale of hundreds of desks per floor.
