# CRM-AUD-EV: crm/audience — event consumers

**Service:** crm/audience · **Build step:** 1 · **Depends on:** CRM-RT-02, CRM-AUD-00 · **Points:** 8

## Goal

crm/audience consumes the events below idempotently, each through its own Pub/Sub subscription, and keeps its tables or
read models current.

## Read first

- `openapi/crm/audience/asyncapi.yaml` (the `from_*` channels and `consume_*` operations) and `openapi/crm/docs/10-events.md`.
- The publisher's message schema for each event (linked from the AsyncAPI document).

## Events (28)

`dealCreated`, `dealLost`, `dealStageChanged`, `dealWon`, `enrolmentCompleted`, `enrolmentFailed`, `enrolmentGoalReached`, `enrolmentStarted`, `enrolmentStepCompleted`, `enrolmentStopped`, `erasureRequested`, `eventRecorded`, `fieldChanged`, `fieldDefined`, `intakeAdmitted`, `leadConverted`, `messageBounced`, `messageClicked`, `messageComplained`, `messageOpened`, `messageSent`, `preferencesReceived`, `scoreChanged`, `siteVisited`, `taskCompleted`, `taskCreated`, `unsubscribeReceived`, `workspaceCreated`

## What to do with them

Update contact_facts and timeline_items (opens, clicks, visits, events, deal stages, score bands, enrolments); `intake.admitted` admits the contact and SUBSCRIBED subscription once per submission id; `unsubscribe.received` / `preferences.received` change subscriptions after verifying the token; `message.bounced` (hard) and `message.complained` suppress and unsubscribe; `erasure.requested` erases.

## Steps

1. One worker binary target or background coroutine group in the impl crate (`src/consumers/`), started from `main.rs`.
2. For each subscription, use rerp_crm_runtime::events::Consumer: set the event's workspace, process in one transaction,
   record `processed_events`, ack after commit; nack on a transient error; dead-letter after 10 attempts.
3. A handler that cannot use an event (unknown version, missing entity) logs it at warn with the event id and acks.

## Acceptance criteria

- AC-1 Delivering every event twice changes nothing the second time.
- AC-2 An event for workspace A never touches B's rows.
- AC-3 A failure inside the handler rolls back and the event is redelivered.
- AC-4 Read models are at most 5 minutes behind under normal load (insights).

## Test cases

| ID | Case | Expected |
|---|---|---|
| TC-1 | Each event, delivered twice | Same state as delivered once |
| TC-2 | Handler fails mid-way | Nothing written; redelivered |
| TC-3 | Event for another workspace | Only that workspace's rows change |

## Verify

```bash
cd microservices && cargo clippy -p rerp_crm_audience --all-targets -- -D warnings && cargo test -p rerp_crm_audience
```
