# ADR 004: Marketing capture is quarantined until confirmed; the CRM store runs on GCP with workspaces

- **Status**: ACCEPTED
- **Date**: 2026-11-09
- **Decision owners**: RERP CRM
- **Group**: CRM data boundary and security
- **Authority**: Normative
- **Scope**: crm.capture-intake-and-workspaces
- **Last reviewed**: 2026-11-09
- **Supersedes**: None
- **Superseded by**: None

## Context

ADR 003 keeps the marketing funnel outside RERP's tenant database and lets
only converted customers cross. It describes the funnel store as the website's
Supabase project, which today holds waiting-list and contact-form submissions
and the pipeline's triage state.

The CRM has since grown into a full marketing suite (PriceWhisperer Epic 55),
and `openapi/crm/docs/08-service-decomposition.md` splits it into seven
services plus a public edge. Three questions were left open there: where the
CRM's own store runs, how anonymous submissions enter it, and whether one
store serves one brand or several. They are decided here.

Known facts:

- Public forms are anonymous internet input. Most newsletter and waiting-list
  sign-ups are never confirmed; some are typos, some are abuse.
- Double opt-in is required for subscriptions in strict regions and is the
  evidence of consent everywhere else.
- PriceWhisperer has one brand. Other users of the CRM manage several brands
  under one organisation.
- Services deploy to GKE Autopilot; GCP resources are created only through the
  gcp-infrastructure Crossplane code.

## Decision

1. **The CRM store is Postgres on GCP.** It is ADR 003's marketing store,
   moved: a Postgres instance on the GCP platform, provisioned by
   gcp-infrastructure through Crossplane, separate from RERP's tenant
   database, with one schema per CRM service. It holds no credential or route
   into RERP; ADR 003's conversion crossing is unchanged.
2. **Unconfirmed input is quarantined.** Anonymous submissions never enter the
   CRM store directly. They land in an **intake store**: a separate, small
   Postgres database written only by the `crm/intake` worker, which holds
   nothing but pending submissions.
3. **The edge receives everything.** Every unauthenticated request (form
   submissions, confirmation clicks, tracking, pixels, click redirects,
   unsubscribe and preferences, provider and ad-platform webhooks, OAuth
   callbacks) reaches `crm/edge`, which has no database credentials. It
   validates shape, origin and rate, and publishes to Pub/Sub. Services pull,
   re-validate and act.
4. **Admission rules.**
   - **Subscription** (newsletter, waiting list, any form with a subscribe
     action): intake stores it as `PENDING` and sends the confirmation email.
     When the person confirms, intake publishes `intake.admitted` with both
     the submission and the confirmation evidence (consent text shown, IP and
     time of submission and of confirmation). `crm/audience` admits the
     contact and the `SUBSCRIBED` subscription in one transaction.
   - **Enquiry** (contact or sales form without a subscribe action): intake
     validates (honeypot, rate, disposable-domain and syntax checks) and
     admits it without confirmation, as a contact with no marketing consent
     and a lead for `crm/sales`. Ticking "also send me news" starts the
     subscription path for that list only.
   - **Staff-entered data** (portal, import, API key) goes straight to the CRM
     store; the user records the consent source, as today.
5. **Intake keeps nothing it does not need.** Pending submissions expire after
   7 days (the confirmation link's life); admitted and rejected ones are
   deleted 30 days after the decision. Intake has no API on the internet.
6. **Workspaces.** One organisation (a Sesame tenant) has one or more
   workspaces, one per brand. Every CRM row carries `workspace_id` and every
   table has row-level security on it. Every CRM API call names its workspace
   in the `X-Workspace-Id` header; the gateway checks it against the caller's
   workspace memberships, which `crm/workspace` owns with a role per
   workspace. Public traffic resolves its workspace from what it targets (the
   form, the tracked site, the signed token). A user with one workspace never
   sees a workspace picker.
7. **Runtime.** Each CRM service, the edge and the intake worker are separate
   Deployments on GKE Autopilot, with Workload Identity for Pub/Sub and the
   databases. The edge scales on request rate; workers scale on their
   subscription backlog.

## Assumptions

- A person who never confirms has given no consent and is not a contact.
- Seven days is long enough to confirm; the link can be re-sent from the
  form's success page or by staff.
- Enquiries need a reply and are legitimate interest; they carry no marketing
  consent until the person gives it.
- Brands that share an organisation do not share contacts: the same person in
  two workspaces is two contacts with two consent records.

## Alternatives considered

### Keep the CRM store in the website's Supabase project

Already in place for leads. Rejected: Supabase is outside the GCP platform's
Crossplane control, its keys would sit in seven services, and the CRM's
transactional and analytical load does not belong in the website's project.

### Admit every submission and mark it unconfirmed

Simpler. Rejected: the CRM store would hold personal data with no consent
behind it, segments and counts would include people who never agreed, and
erasure would have to chase noise.

### Stateless confirmation (the whole submission inside a signed token)

No intake store at all. Rejected for now: re-sending a confirmation, rate
limiting per address and showing staff what is pending all need state, and the
token would carry personal data in a URL.

### One store per brand

Strong isolation. Rejected: an organisation with several brands would run
several instances; RLS on `workspace_id` gives the separation with one
instance, and a brand can still be moved to its own instance later.

## Consequences

### Positive

- The CRM store holds only people who confirmed, enquired or were entered by
  staff, which is what segments, counts and consent evidence assume.
- The internet reaches no process with a database credential.
- Several brands run on one installation without sharing contacts.

### Costs and risks

- A ninth component (`crm/intake`) and a second small database.
- Two hops before a sign-up appears in the CRM (edge → intake → audience);
  admission after confirmation should still be visible within seconds.
- Every API call carries `X-Workspace-Id`; a missing or foreign workspace is
  a 403 (or 404 for a record), never a default.
- The website's forms move from Supabase to the edge. Until they do, intake
  also reads the Supabase capture tables (with the same read-only key the
  pipeline uses today), so ADR 003's funnel reader moves from `crm/pipeline`
  to `crm/intake`.

## Implementation and verification

1. **Contracts.** `openapi/crm/edge` (public operations), `openapi/crm/intake`
   (staff view of pending submissions), the `X-Workspace-Id` header on every
   other CRM operation, and `crm/workspace` workspaces and memberships.
2. **Platform** (gcp-infrastructure): the CRM Postgres instance and the intake
   database, Pub/Sub topics and subscriptions with dead-letter queues, service
   accounts and Workload Identity bindings for each Deployment.
3. **Admission.** `crm/audience` consumes `intake.admitted`: schema, size,
   allowlist, once-only on submission id, contact and subscription in one
   transaction, ack after commit.

Acceptance:

- A sign-up that is never confirmed leaves no row in the CRM store, and none
  in intake after 7 days.
- A confirmed sign-up appears as a subscribed contact with the consent text
  and both timestamps.
- Replaying `intake.admitted` creates nothing new.
- A request with another workspace's id returns 403; a record from another
  workspace returns 404.
- No CRM service, the edge or intake holds a RERP tenant database credential.

## Related requirements

- PriceWhisperer Epic 55: US_55_06 (lists and consent), US_55_14 (forms),
  US_55_39 (tenancy, roles and audit).
- EPIC-32 US_32_02 (verification email), whose flow this generalises.

## Related decisions and artifacts

- ADR 003: the funnel stays outside RERP; only converted customers cross. Still
  in force; this ADR decides where the funnel store runs and how input enters
  it.
- `openapi/crm/docs/08-service-decomposition.md`: the service split.
- `openapi/crm/edge`, `openapi/crm/intake`, `openapi/crm/workspace`.
