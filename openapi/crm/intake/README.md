# crm/intake — the quarantine (ADR 004)

The quarantine for anonymous submissions (ADR 004). A worker keeps pending submissions in its own database until they are confirmed (subscriptions) or validated (enquiries), sends the confirmation email, and publishes intake.admitted for crm/audience. This API is the staff view of what is pending; nothing here is on the internet.

| | |
|---|---|
| **Contract** | [openapi.yaml](openapi.yaml) (4 operations), [asyncapi.yaml](asyncapi.yaml) |
| **Base path** | `/api/v1/crm/intake`, `X-Workspace-Id` on every call |
| **Generated crate** | `microservices/crm/intake/gen` (`rerp_crm_intake_gen`) |
| **Implementation** | Not started; the generated crate serves the spec examples. Build step 2 in [13-implementation-plan.md](../docs/13-implementation-plan.md). |
| **Replaces** | nothing (new, ADR 004) |

## Design

From [08-service-decomposition.md §4](../docs/08-service-decomposition.md):

| | |
|---|---|
| **Owns** | Pending submissions in their own small database: the fields, consent text shown, form and workspace, submission IP and time, the confirmation token and its expiry; the decision (admitted, rejected, expired). |
| **Does** | Consumes `form.submission.received` from the edge; screens (honeypot, rate per address and IP, syntax and disposable domains); sends the double opt-in email for subscriptions; on `confirmation.received` (or immediately, for an enquiry) publishes `intake.admitted` with the submission and its evidence. Until the website's forms post to the edge, it also reads the EPIC-32 Supabase capture tables. |
| **Never** | Writes to the CRM store, or keeps anything past 7 days pending or 30 days decided. |
| **API** | Staff-only, workspace-scoped: list pending submissions, re-send a confirmation, discard one. Nothing on the internet. |

## API

| Area | Operations |
|---|---|
| `submissions` — Submissions | 4 |

Roles per operation: [11-permissions.md](../docs/11-permissions.md#intake). Conventions: [09-api-conventions.md](../docs/09-api-conventions.md).

## Events

Publishes on `crm.intake.v1`:

- `erasure.completed` — A service erased its copy for a data request.
- `intake.confirmation_requested` — Messaging sends the double opt-in email (transactional, no consent needed).
- `intake.admitted` — A confirmed subscription or a screened enquiry, with its consent evidence (ADR 004).

Consumes: `ad_webhook.received`, `confirmation.received`, `erasure.requested`, `event_registration.received`, `form_submission.received`, `survey.received`

Catalogue and transport: [10-events.md](../docs/10-events.md).

## Synchronous calls

- None: it consumes edge events and publishes `intake.admitted`; the confirmation email goes out through messaging via `intake.confirmation_requested`.
