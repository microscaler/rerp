# Workplace Desk and Room Booking PRD

- **Status**: DRAFT
- **Authority**: Working
- **Owner**: RERP maintainers
- **Scope**: workplace.booking-product
- **Last reviewed**: 2026-09-18
- **Supersedes**: None
- **Superseded by**: None

## Purpose

Deliver a desk, room, parking, and pool-vehicle booking product on RERP that a
public administration can adopt for desk sharing. It is RERP's first
end-to-end showcase: a real requirement, a real UI, and real use of Sesame
IDAM, RBAC, Documents, and Analytics.

Architecture boundary: [ADR 003](../adrs/003-workplace-booking-suite-boundary.md).

## Requirement sources

| Source | Reference | Reviewed | Use |
|---|---|---|---|
| Stadt Chemnitz, "Raumbuchungsprogramm für Desksharing" | TED 595241-2026, internal ID 17/26/0122 | 2026-09-18, public notice only | Primary external requirement |
| Established desk-booking products | Public product descriptions | 2026-09-18 | Market baseline evaluators will compare against |
| Clean-room study of open-source floor planners | Published concepts only; no code copied | 2026-09-18 | Floor-plan editor design |

The Chemnitz specification documents (Leistungsbeschreibung, evaluation
catalogue, price sheet) are published on evergabe.de behind free registration
and have **not yet been reviewed**. Requirements marked *Assumed* must be
confirmed or removed once they are.

### Known from the Chemnitz notice

- Configurable bookable resources: workstations, meeting rooms, possibly
  vehicles.
- Not only booking: an analysis function for optimising office space.
- About 1,800 resource-based licences used by about 5,000 users; about 420 at
  initial rollout, the rest by about 2030.
- Hosting, setup, support, training, and an annual budget for custom
  development.
- Open-ended cloud contract (EVB-IT Cloud), renewing annually.
- Award: 50% technical value, 50% price. Documents and bids in German.
- Selection: at least three comparable references with contact, scope, value,
  and period.

## Users and roles

- **Employee**: finds and books a resource, checks in, cancels, sees colleagues
  where privacy allows.
- **Team lead / delegate**: books for others and for a team.
- **Facility administrator**: maintains sites, floors, plans, zones, resources,
  policies, and blocks; reads utilisation reports.
- **Tenant administrator**: identity-provider connection, roles, and access
  rules.

## Functional requirements

### Space model and floor-plan editor (Spaces)

1. **FR-S1** Sites, buildings, and floors with IANA time zones and ordinal
   levels.
2. **FR-S2** Upload a plan (image or PDF page) to Documents and calibrate it by
   marking two points and entering the real distance; set rotation, origin, and
   opacity.
3. **FR-S3** Trace walls over the background as a graph of shared corners;
   moving a corner moves every attached wall. Wall kinds: structural,
   partition, glass, virtual.
4. **FR-S4** Doors and windows are hosted on walls by offset and width.
5. **FR-S5** Rooms are derived automatically from closed wall loops, with
   computed area; administrators name and classify rooms but do not draw them.
6. **FR-S6** Snapping to grid, endpoints, and 0/45/90 degrees, with live
   dimension labels.
7. **FR-S7** Zones as polygons with access rules by group or role.
8. **FR-S8** Resources (desk, meeting room, parking space, vehicle, locker)
   with geometry, capacity, attribute tags, and assignment mode (bookable,
   permanently assigned, not bookable). Vehicles may be off-plan.
9. **FR-S9** Bulk placement of a grid of resources with a label pattern.
10. **FR-S10** Saving a plan uses optimistic concurrency; geometry changes never
    silently delete resources and report resources left outside rooms.
11. **FR-S11** Draft and published floors; only published floors are bookable.

### Booking (Bookings)

1. **FR-B1** Book a resource for a time window; overlapping bookings are
   rejected.
2. **FR-B2** Weekly recurring bookings for fixed office days; conflicting
   occurrences are skipped and reported, not failed as a whole.
3. **FR-B3** Group bookings for several people and resources, created
   atomically.
4. **FR-B4** Book on behalf of another person with a delegate permission.
5. **FR-B5** Check-in by web, QR code at the resource, NFC, or kiosk; automatic
   release of bookings not checked in within the policy window; early
   check-out. QR and NFC credentials are proof-of-presence secrets: only a
   facility administrator may retrieve or rotate them, and ordinary resource
   responses never expose them.
6. **FR-B6** Booking policies scoped to tenant, site, building, floor, zone,
   resource, or resource kind: booking horizon, maximum duration, maximum
   active bookings, check-in requirement, working hours, recurrence allowed.
   Each policy selects exactly one scope. Precedence is resource, zone, floor,
   building, site, resource kind, then tenant; conflicting active policies at
   the same scope are rejected.
7. **FR-B7** Administrative blocks (maintenance, events) with optional
   cancellation and notification of affected bookings.
8. **FR-B8** Private bookings hide the occupant's name from other users.
9. **FR-B9** Availability for a time window can be queried by floor or explicit
   resource identifiers. It drives both the map and an equivalent list view,
   and supports off-plan resources such as pool vehicles.

### Analytics

1. **FR-A1** Daily utilisation facts per resource: available, booked, and
   occupied (checked-in) minutes, and no-show count.
2. **FR-A2** Heatmap on the floor plan using the same geometry as booking.
3. **FR-A3** Space-optimisation reports by zone, floor, and weekday
   (delivered through the Analytics suite).

### Integration

1. **FR-I1** Sign-in through the tenant's identity provider via Sesame IDAM.
   *Assumed*: Microsoft Entra ID or Active Directory for Chemnitz.
2. **FR-I2** *Assumed*: meeting-room bookings synchronised with Outlook/Teams
   calendars through Microsoft Graph.
3. **FR-I3** Bookings publishes confirmation, reminder, release, and block-
   cancellation lifecycle events. The Notifications suite owns email delivery;
   Workplace has no synchronous dependency on it.
4. **FR-I4** Resource import and export for the Chemnitz-scale rollout is a
   post-MVP contract. CSV shape, validation reporting, and asynchronous job
   semantics must be specified after the full tender documents are reviewed.

## Non-functional requirements

1. **NFR-1 Accessibility**: conformance with BITV 2.0 / WCAG 2.1 AA. The map is
   never the only way to book: every map action has a list or table
   equivalent, and the map is keyboard navigable with labelled elements (SVG
   rendering is preferred over raster canvas for this reason).
2. **NFR-2 Language**: German and English UI; German documentation.
3. **NFR-3 Data protection**: GDPR compliant; hosting in the EU; configurable
   retention for booking history; utilisation reports aggregate and do not
   profile individuals.
4. **NFR-4 Scale**: at least 5,000 users and 2,000 resources per tenant;
   availability for one floor returns in under 300 ms at the 95th percentile.
5. **NFR-5 Security**: tenant isolation, least-privilege roles, audit trail of
   administrative changes. *Assumed*: ISO 27001 or BSI C5 evidence requested
   under EVB-IT Cloud.
6. **NFR-6 Mobile**: responsive web that works on phones for booking and QR
   check-in; a native app is out of scope for the first release.

## Out of scope for the first release

- 3D views and full CAD drafting; plans are traced over uploaded drawings.
- Native mobile apps.
- Sensor-based occupancy (IoT suite may add this later).
- Visitor management and catering.
- Bulk resource import/export; the first release supports API-driven bulk
  placement, while the CSV contract remains pending tender review.

## Acceptance criteria (first release)

1. An administrator uploads a plan, calibrates it, traces walls, and sees rooms
   derived automatically with correct areas (±1%).
2. An administrator places a grid of 40 desks in one operation and publishes
   the floor.
3. An employee signed in through the identity provider books a desk on the map
   and, independently, through the list view using only the keyboard.
4. A second booking for an overlapping window on the same desk is rejected.
5. A booking not checked in within the policy window is released and the desk
   becomes free on the map.
6. A user outside a zone's allowed groups sees its desks as restricted and
   cannot book them.
7. The utilisation heatmap for the floor reflects bookings and check-ins from
   the previous day.
8. A normal employee cannot retrieve a QR or NFC check-in credential through
   resource list or detail APIs; a facility administrator can rotate it and the
   previous credential stops working immediately.

## Open decisions

1. **Licence of the Workplace application.** German and Bulgarian public buyers
   may require delivered software to be open source. Options: release the
   Workplace suite under an open licence such as EUPL-1.2 while the RERP
   platform keeps PolyForm Shield, or keep a single licence. Requires a
   business decision before the first public release.
2. **Calendar integration depth** (FR-I2): confirm against the Chemnitz
   specification.
3. **Licence metric**: Chemnitz prices per resource, not per user. Pricing and
   metering design is pending.
4. **Service count**: ADR 003 proposes two services; confirm after the first
   implementation spike.
