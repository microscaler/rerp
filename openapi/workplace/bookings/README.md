# Workplace Bookings

> **Status: PROPOSED** contract, version 0.1.0. See
> [ADR 003](../../../docs/adrs/003-workplace-booking-suite-boundary.md).

## Responsibility

Owns reservations of resources defined by Workplace Spaces:

- **Bookings** with overlap rejection (`409`), weekly recurrence (conflicting
  occurrences are skipped and reported), atomic group bookings, and booking on
  behalf of others.
- **Check-in** by web, QR, NFC, or kiosk; automatic release of no-shows; early
  check-out.
- **Policies** scoped from tenant down to a single resource; the most specific
  scope wins.
- **Administrative blocks** with optional cancellation of affected bookings.
- **Availability** for one floor and time window. It feeds both the map and the
  accessible list view and never reveals names on private bookings.
- **Utilisation facts** per resource and day for heatmaps and reports.

Resource identity and access rules are read from Spaces over its API.
