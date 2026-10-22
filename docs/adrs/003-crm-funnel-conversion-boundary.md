# ADR 003: The marketing funnel stays outside RERP; only converted customers cross

- **Status**: ACCEPTED
- **Date**: 2026-10-23
- **Decision owners**: RERP CRM and Accounting
- **Group**: CRM data boundary and security
- **Authority**: Normative
- **Scope**: crm.funnel-conversion-boundary
- **Last reviewed**: 2026-10-23
- **Supersedes**: None
- **Superseded by**: None

## Context

Public website forms (waiting list, contact form) write to the marketing
database, a Supabase project the website owns. It was built so that an
external input path never reaches RERP's backend database. RERP's
`crm/pipeline` service reads those submissions as leads and keeps triage state
(`crm_lead_state`) in the same project. It holds the Supabase key and no RERP
database credentials.

Accounting now needs a customer record it can invoice, chase and report on
(invoice `customer_id`, AR statements, follow-ups). That record must live in
RERP's database, under tenant RLS. Most funnel entries never buy; they are
noise for the ERP and personal data RERP has no reason to hold.

PriceWhisperer solved the same shape for Stripe (PriceWhisperer ADR-0005):
Stripe reaches only a keyless relay that publishes to Pub/Sub. Billing pulls
from inside the cluster, treats every message as untrusted and re-retrieves
the event from Stripe before acting.

## Decision

1. **The funnel is an external store.** The marketing database holds
   submissions, pipeline triage (stages, notes), consent and marketing
   history. It is not part of RERP's database, and nothing in it holds a
   credential or a network route into RERP. Prospects are emailed and
   nurtured from there.
2. **Only converted customers cross.** A lead converts when RERP will have to
   invoice them or record a payment from them:
   - a paid Stripe checkout (automatic);
   - an accepted quote or order, or a signed contract, marked *won* in the
     pipeline by staff, who supply the billing details (manual, infrequent,
     acceptable for now and open to revisiting).
   Waiting-list signups, enquiries and open opportunities never cross.
3. **One crossing, by pull.** A conversion is a message on the Pub/Sub topic
   `crm-conversions`. Only two identities may publish: the pipeline service
   (manual conversions) and PriceWhisperer billing (Stripe conversions). RERP
   pulls the subscription from inside the cluster; nothing outside dials in.
4. **Intake treats every message as untrusted.**
   - Size limit, schema and field allowlist; anything else is acked and dropped.
   - Re-verification at the source before belief:
     - a Stripe conversion is re-retrieved from Stripe by id with a restricted,
       read-only key;
     - a manual conversion carries an Ed25519 signature by the pipeline over
       the canonical payload; intake holds only the public key. RERP needs no
       credential for the funnel at all.
   - Once-only: a conversion ledger keyed by conversion id. A second conversion
     of the same funnel lead or Stripe customer links to the existing customer
     and creates nothing new.
5. **The customer record is the CRM account.** CRM `accounts` owns it in
   RERP's database (Odoo's `res.partner` role). Accounting refers to it by id
   (invoice `customer_id`, AR open items) and only reads it.
6. **Minimal crossing payload.** Legal or company name, contact name, billing
   email, billing address, country, tax id when known, the conversion reason
   and its reference (Stripe customer and checkout ids, or the order/contract
   reference), the funnel lead id when there is one, and who converted it.
   Marketing history, notes and consent stay in the funnel.
7. **No write-back.** RERP never writes to the funnel. The pipeline marks a
   lead converted itself when it publishes. After conversion the person exists
   in both stores: the funnel for marketing under their consent, RERP as a
   customer.

## Assumptions

- Manual conversions stay rare enough that typing billing details is no burden.
- One funnel and one tenant per brand. Each conversion carries the tenant it
  converts into, and intake rejects tenants it does not serve.
- Pub/Sub IAM is the publisher check. The pipeline's signature, or Stripe's
  own record, is the second, independent check.

## Alternatives considered

### Import every submission into RERP on intake

This keeps all sales work inside RERP, with one crossing at intake. It was
rejected because RERP would hold unconverted personal data and noise. The
funnel is already where marketing happens.

### Let the funnel or website write to RERP (webhook or direct database access)

This would give an external input path a route or a credential into the
backend, the exposure the funnel was built to avoid. It was rejected.

### Keep the customer master in Supabase and have accounting read it

Supabase can't be joined with RERP data, isn't under RERP's tenant RLS, and
would put invoiceable customer records in the externally exposed store.
Rejected.

## Consequences

### Positive

- The internet-facing store holds no key into RERP, and RERP holds no key
  into the funnel. Outward, RERP holds only a restricted, read-only Stripe key.
- RERP holds only customers it has a contractual or accounting reason to keep.
- One audited crossing, idempotent and verified at the source.

### Costs and risks

- Two stores hold the same person after conversion. Contact changes after
  conversion are made in RERP. The funnel copy is marketing-only.
- An erasure request is handled in both stores. RERP may keep what accounting
  law requires.
- New platform pieces: a RERP database on the data plane, the topic and its
  subscription, publisher and subscriber service accounts, and a restricted
  Stripe key for RERP.
- Manual conversions depend on staff entering billing details correctly.
  Intake validates format, not truth.

## Implementation and verification

1. **Contract.** The conversion message schema (versioned) and the pipeline's
   `convert_lead` operation, which records *won* with billing details,
   publishes, and marks the lead converted in the funnel.
2. **Customer record.** The CRM account entity and migration in RERP's
   database: tenant RLS, unique per funnel lead and per Stripe customer, and
   the conversion ledger.
3. **Intake.** The CRM accounts consumer: pull, screen, re-verify, once-only
   write, ack after commit. Transient failures nack and retry; after 10
   attempts the message goes to `crm-conversions-dlq`, which pages.
4. **Platform** (gcp-infrastructure, through Crossplane):
   - the RERP database and its credentials;
   - the topic, subscription and dead-letter queue;
   - the pipeline's signing key, with its public half in the intake's config;
   - service accounts and their Workload Identity bindings;
   - the restricted Stripe key;
   - the crm-accounts deployment.
5. **Stripe path.** PriceWhisperer billing publishes a conversion on a
   completed checkout.

Acceptance:

- A manual *won* conversion creates exactly one account.
- Replaying the message creates nothing.
- A forged message (a bad or missing signature, an altered payload, an
  unknown Stripe id) creates nothing and is dropped.
- No RERP credential exists in the funnel or the website. No funnel
  credential exists in any RERP service other than the pipeline.

## Related decisions and artifacts

- ADR 001: accounting runtime boundary.
- PriceWhisperer ADR-0005: Stripe webhook relay and Pub/Sub intake.
- `microservices/crm/pipeline/impl/src/supabase.rs`: today's funnel reader.
- `openapi/crm/accounts/openapi.yaml`: the customer (account) contract.

## Notes

- 2026-10-23: before any implementation, manual-conversion verification moved
  from re-reading the funnel with a read-only key to a pipeline signature. It
  proves the same origin without RERP holding any funnel credential.
