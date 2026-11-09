# CRM-RT-02: rerp_crm_runtime — events: envelope, outbox, relay, consumer

**Crate:** `microservices/crm/runtime` (module `events`) · **Build step:** 0 · **Depends on:** CRM-RT-01, CRM-DB-01 · **Points:** 8

## Goal

Events leave every service through a transactional outbox and arrive through an idempotent consumer, exactly as
`openapi/crm/docs/10-events.md` says.

## Read first

- `openapi/crm/docs/10-events.md` (Transport) and one `asyncapi.yaml` (the envelope schema in `components.messages`).
- `microservices/crm/migrations/templates/rls_and_infrastructure.sql` (`outbox`, `processed_events`).
- `microservices/crm/conversion/src/pubsub.rs`: how RERP already talks to Pub/Sub (auth, publish, pull, ack).

## API to provide

```rust
pub struct Envelope<T> { pub event_id: Uuid, pub event_type: String, pub workspace_id: Uuid, pub occurred_at: DateTime<Utc>,
                         pub producer: String, pub actor: Actor, pub request_id: Option<String>, pub entity_id: String,
                         pub data: T }
/// Inside the caller's transaction: one outbox row; event_id is a new UUID v7.
pub fn append<T: Serialize>(tx, ctx: &RequestContext, event_type: &str, entity_id: &str, data: &T) -> Result<Uuid, ApiError>;
/// Background loop: read unpublished rows (oldest first, batch 100), publish to topic crm.<service>.v1 with attributes
/// event_type and workspace_id and ordering key entity_id, mark published_at; on error increment attempts, keep last_error.
pub fn run_relay(service: &str, pool, publisher: &dyn Publisher);
/// Pull from a subscription; for each message: parse the envelope, set the workspace, run `handle` and insert
/// processed_events(event_id) in one transaction; ack after commit; skip (ack) when event_id was already processed;
/// nack on error (Pub/Sub retries with backoff and dead-letters after 10 attempts).
pub fn run_consumer(subscription: &str, pool, handle: impl Fn(&mut Tx, &Envelope<serde_json::Value>) -> Result<(), ApiError>);
pub trait Publisher { fn publish(&self, topic: &str, data: &[u8], attributes: &[(&str, &str)], ordering_key: &str) -> Result<(), String>; }
```

A `Publisher` for tests records messages in memory; the real one uses the Pub/Sub REST API with Workload Identity
(token from the metadata server), as `crm/conversion` does.

## Acceptance criteria

- AC-1 A change that rolls back leaves no outbox row; one that commits is published exactly once by the relay (even if
  the relay restarts between publish and marking, consumers dedupe on event_id).
- AC-2 Envelopes validate against the AsyncAPI message schema of the event.
- AC-3 Delivering the same message twice to a consumer runs `handle` once.
- AC-4 A failing handler writes nothing and the message is nacked.

## Test cases

| ID | Case | Expected |
|---|---|---|
| TC-1 | append + rollback | No outbox row |
| TC-2 | append + commit + relay with the memory publisher | One message, attributes and ordering key set, row marked published |
| TC-3 | Consumer receives the same event twice | Handler ran once |
| TC-4 | Handler returns an error | Nothing committed; nack |

## Verify

```bash
cd microservices && cargo test -p rerp_crm_runtime events && cargo clippy -p rerp_crm_runtime --all-targets -- -D warnings
```
