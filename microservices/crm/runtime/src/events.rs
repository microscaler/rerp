//! The event envelope (`event_id`, `event_type`, `workspace_id`, `occurred_at`, `producer`, `actor`,
//! `request_id`, `entity_id`, `data`), the outbox writer (same transaction as the change), the relay (publish
//! after commit, Pub/Sub attributes `event_type` and `workspace_id`, ordering key `entity_id`), and the
//! idempotent consumer (`processed_events`, ack after commit, nack to retry). openapi/crm/docs/10-events.md.
//!
//! All table access is through the services' own Lifeguard entities: a service implements [`OutboxTable`] and
//! [`ProcessedTable`] over its `<schema>.outbox` / `<schema>.processed_events` (the shapes the migrations
//! template provisions). No SQL is written here except the session-scoped `crm.workspace_id` guard from
//! [`crate::context::workspace_guard_sql`].
//!
//! Story: CRM-RT-02 (docs/crm/stories/).

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Duration;

use chrono::{DateTime, Utc};
use lifeguard::raw_sql::execute_statement;
use lifeguard::transaction::Transaction;
use lifeguard::{LifeExecutor, MayPostgresExecutor};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::context::{ActorKind, RequestContext};
use crate::errors::ApiError;
use crate::pubsub::{PubSub, Received};

/// The batch size the relay publishes and the consumer pulls per round (CRM-RT-02 API contract).
pub const EVENT_BATCH_SIZE: u32 = 100;

/// A database/infrastructure failure inside the event machinery (never a caller-visible domain error).
/// Exported for services' [`OutboxTable`]/[`ProcessedTable`] impls to map their `LifeError`s the same way.
#[must_use]
pub fn event_db_error(error: impl std::fmt::Display) -> ApiError {
    ApiError {
        status: 500,
        reason: "internal_error".to_string(),
        message: "The event infrastructure failed".to_string(),
        details: Some(error.to_string()),
        fields: Vec::new(),
    }
}

/// The envelope every CRM message carries around `data` (`10-events.md`, each `asyncapi.yaml`'s payload
/// schema). The serialized field names and the required/optional split ARE the contract; the tests below
/// assert them against the generated `asyncapi.yaml` documents (AC-2).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Envelope<T> {
    pub event_id: Uuid,
    pub event_type: String,
    pub workspace_id: Uuid,
    pub occurred_at: DateTime<Utc>,
    pub producer: String,
    pub actor: Actor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    pub entity_id: String,
    pub data: T,
}

/// Who caused the event: `kind` is the contract enum (`USER`, `API_KEY`, `SYSTEM`, `CONTACT`, `ANONYMOUS`).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Actor {
    pub kind: String,
    pub id: String,
}

impl Actor {
    fn from_context(ctx: &RequestContext) -> Self {
        // The envelope's contract enum is wider (CONTACT, ANONYMOUS for untrusted edge events), but a
        // producer inside a service is always a user, an API key or its own worker (`context::ActorKind`).
        let kind = match ctx.actor_kind {
            ActorKind::User => "USER",
            ActorKind::ApiKey => "API_KEY",
            ActorKind::System => "SYSTEM",
        };
        Actor {
            kind: kind.to_string(),
            id: ctx.actor_id.clone(),
        }
    }
}

/// The publish side: one message body, its filter attributes and its ordering key.
pub trait Publisher {
    fn publish(
        &self,
        topic: &str,
        data: &[u8],
        attributes: &[(&str, &str)],
        ordering_key: &str,
    ) -> Result<(), String>;
}

/// One message pulled from a subscription.
#[derive(Clone, Debug)]
pub struct InboundMessage {
    pub ack_id: String,
    pub data: Vec<u8>,
    pub delivery_attempt: u32,
}

/// The consume side of the transport.
pub trait Subscriber {
    fn pull(&mut self, subscription: &str, max: u32) -> Result<Vec<InboundMessage>, String>;
    fn ack(&mut self, subscription: &str, ack_ids: &[String]) -> Result<(), String>;
    fn nack(&mut self, subscription: &str, ack_ids: &[String]) -> Result<(), String>;
}

/// Lock a shared client's mutex without failing the service when a panicking thread poisoned it.
fn shared<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

/// The real publisher: the shared Pub/Sub REST client with Workload Identity ([`crate::pubsub`]).
/// Topics are full resource names — build them with [`topic_for`].
pub struct PubSubPublisher {
    pubsub: Mutex<PubSub>,
}

impl PubSubPublisher {
    /// Errors only when the client cannot be built (a malformed `PUBSUB_EMULATOR_HOST`).
    pub fn new() -> Result<Self, String> {
        Ok(PubSubPublisher {
            pubsub: Mutex::new(PubSub::new()?),
        })
    }
}

impl Publisher for PubSubPublisher {
    fn publish(
        &self,
        topic: &str,
        data: &[u8],
        attributes: &[(&str, &str)],
        ordering_key: &str,
    ) -> Result<(), String> {
        shared(&self.pubsub)
            .publish_ordered(topic, data, attributes, ordering_key)
            .map(|_message_id| ())
    }
}

/// The real subscriber: the same client, pull / ack / nack.
pub struct PubSubSubscriber {
    pubsub: Mutex<PubSub>,
}

impl PubSubSubscriber {
    /// Errors only when the client cannot be built (a malformed `PUBSUB_EMULATOR_HOST`).
    pub fn new() -> Result<Self, String> {
        Ok(PubSubSubscriber {
            pubsub: Mutex::new(PubSub::new()?),
        })
    }
}

impl Subscriber for PubSubSubscriber {
    fn pull(&mut self, subscription: &str, max: u32) -> Result<Vec<InboundMessage>, String> {
        let received: Vec<Received> = shared(&self.pubsub).pull(subscription, max)?;
        Ok(received
            .into_iter()
            .map(|r| InboundMessage {
                ack_id: r.ack_id,
                data: r.data,
                delivery_attempt: r.delivery_attempt,
            })
            .collect())
    }

    fn ack(&mut self, subscription: &str, ack_ids: &[String]) -> Result<(), String> {
        shared(&self.pubsub).ack(subscription, ack_ids)
    }

    fn nack(&mut self, subscription: &str, ack_ids: &[String]) -> Result<(), String> {
        shared(&self.pubsub).nack(subscription, ack_ids)
    }
}

/// One `<schema>.outbox` row (`rls_and_infrastructure.sql`), the shared shape every service entity maps to.
#[derive(Clone, Debug)]
pub struct OutboxRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub event_type: String,
    pub entity_id: String,
    /// The fully built envelope JSON; the relay publishes it verbatim as the message body.
    pub payload: Value,
    pub occurred_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub attempts: i32,
    pub last_error: Option<String>,
}

/// A service's `<schema>.outbox` Lifeguard entity, mapping its Model/Record to [`OutboxRow`].
/// The relay's read needs no workspace GUC: the template provisions `outbox` with the documented permissive
/// relay policy (the service's own schema is the isolation boundary there), while an append inside a request
/// transaction still runs under the caller's guard.
pub trait OutboxTable {
    /// The envelope's `producer`, e.g. `crm/audience` — the asyncapi `producer` const.
    const PRODUCER: &'static str;

    /// Insert one row on the caller's executor: it commits or rolls back with the change (AC-1).
    fn insert<E: LifeExecutor>(exec: &E, row: &OutboxRow) -> Result<(), ApiError>;

    /// Unpublished rows, oldest `occurred_at` first, at most `limit` (the partial index serves this).
    fn unpublished<E: LifeExecutor>(exec: &E, limit: u64) -> Result<Vec<OutboxRow>, ApiError>;

    fn mark_published<E: LifeExecutor>(
        exec: &E,
        id: Uuid,
        published_at: DateTime<Utc>,
    ) -> Result<(), ApiError>;

    /// A failed publish: increment attempts, keep the reason; the row stays unpublished for the next round.
    fn record_failure<E: LifeExecutor>(
        exec: &E,
        id: Uuid,
        last_error: &str,
    ) -> Result<(), ApiError>;
}

/// A service's `<schema>.processed_events` entity: the consumer's idempotency ledger.
pub trait ProcessedTable {
    fn is_processed<E: LifeExecutor>(exec: &E, event_id: Uuid) -> Result<bool, ApiError>;

    /// Insert the marker inside the SAME transaction as the handler's writes.
    fn record<E: LifeExecutor>(exec: &E, event_id: Uuid, event_type: &str) -> Result<(), ApiError>;
}

/// The transaction operations the consumer needs, abstract so the tests can drive [`consume_batch`] without
/// a database while services use the real [`Transaction`] implementation.
pub trait CrmTx: LifeExecutor {
    /// Set the transaction-local `crm.workspace_id` ([`crate::context::workspace_guard_sql`]) so RLS
    /// applies; the consumer runs this before touching `processed_events` or calling the handler.
    fn apply_workspace_guard(&self, workspace_id: Uuid) -> Result<(), ApiError>;

    fn commit(self) -> Result<(), ApiError>;

    fn rollback(self);
}

impl CrmTx for Transaction {
    fn apply_workspace_guard(&self, workspace_id: Uuid) -> Result<(), ApiError> {
        execute_statement(
            self,
            crate::context::workspace_guard_sql(),
            &[&workspace_id],
        )
        .map(|_| ())
        .map_err(event_db_error)
    }

    fn commit(self) -> Result<(), ApiError> {
        Transaction::commit(self).map_err(event_db_error)
    }

    fn rollback(self) {
        Transaction::rollback(self).ok();
    }
}

/// Where the consumer starts its transaction. Services pass their dedicated worker connection
/// (a background loop should not share the request pool), but any [`CrmTx`] source works.
pub trait TxSource {
    type Tx: CrmTx;

    fn begin_crm_tx(&self) -> Result<Self::Tx, ApiError>;
}

impl TxSource for MayPostgresExecutor {
    type Tx = Transaction;

    fn begin_crm_tx(&self) -> Result<Self::Tx, ApiError> {
        MayPostgresExecutor::begin(self).map_err(event_db_error)
    }
}

/// Build the envelope for one event; `event_id` is a fresh UUID v7 (time-ordered, so id order matches
/// outbox age). Public so a service can inspect or store the id before writing its own side tables.
pub fn build_envelope<O: OutboxTable, T: Serialize>(
    ctx: &RequestContext,
    event_type: &str,
    entity_id: &str,
    data: &T,
) -> Result<Envelope<Value>, ApiError> {
    Ok(Envelope {
        event_id: Uuid::now_v7(),
        event_type: event_type.to_string(),
        workspace_id: ctx.workspace_id,
        occurred_at: Utc::now(),
        producer: O::PRODUCER.to_string(),
        actor: Actor::from_context(ctx),
        request_id: ctx.request_id.clone(),
        entity_id: entity_id.to_string(),
        data: serde_json::to_value(data).map_err(event_db_error)?,
    })
}

/// Write the outbox row for one event INSIDE the caller's transaction (`tx` is the request's executor):
/// a rollback leaves no row (AC-1); a commit makes the event visible to the relay. Returns the `event_id`.
pub fn append<O: OutboxTable, T: Serialize, E: LifeExecutor>(
    tx: &E,
    ctx: &RequestContext,
    event_type: &str,
    entity_id: &str,
    data: &T,
) -> Result<Uuid, ApiError> {
    let envelope = build_envelope::<O, T>(ctx, event_type, entity_id, data)?;
    let row = OutboxRow {
        id: envelope.event_id,
        workspace_id: envelope.workspace_id,
        event_type: envelope.event_type.clone(),
        entity_id: envelope.entity_id.clone(),
        payload: serde_json::to_value(&envelope).map_err(event_db_error)?,
        occurred_at: envelope.occurred_at,
        published_at: None,
        attempts: 0,
        last_error: None,
    };
    O::insert(tx, &row)?;
    Ok(row.id)
}

/// The topic one publishing service owns: `crm.<service>.v1` (`10-events.md`), as a Pub/Sub resource name.
#[must_use]
pub fn topic_for(project: &str, service: &str) -> String {
    format!("projects/{project}/topics/crm.{service}.v1")
}

/// What one relay round did.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RelayReport {
    pub published: usize,
    pub failed: usize,
}

/// One relay round: take up to [`EVENT_BATCH_SIZE`] unpublished rows oldest-first, publish each to
/// `crm.<service>.v1` with the filter attributes (`event_type`, `workspace_id`, `schema_version`) and
/// `entity_id` as the ordering key, then mark it published. A failed publish records the reason and the row
/// retries next round; if the relay dies between publish and mark, the redelivery dedupes on `event_id`
/// (AC-1's at-least-once half).
pub fn relay_batch<O: OutboxTable, E: LifeExecutor, P: Publisher + ?Sized>(
    exec: &E,
    project: &str,
    service: &str,
    publisher: &P,
) -> Result<RelayReport, ApiError> {
    let topic = topic_for(project, service);
    let rows = O::unpublished(exec, u64::from(EVENT_BATCH_SIZE))?;
    let mut report = RelayReport::default();
    for row in rows {
        let body = serde_json::to_vec(&row.payload).map_err(event_db_error)?;
        let workspace_id = row.workspace_id.to_string();
        let attributes: Vec<(&str, &str)> = vec![
            ("event_type", row.event_type.as_str()),
            ("workspace_id", workspace_id.as_str()),
            ("schema_version", "1"),
        ];
        match publisher.publish(&topic, &body, &attributes, &row.entity_id) {
            Ok(()) => {
                O::mark_published(exec, row.id, Utc::now())?;
                report.published += 1;
            }
            Err(error) => {
                let reason = format!("publish to {topic}: {error}");
                O::record_failure(exec, row.id, &reason)?;
                report.failed += 1;
            }
        }
    }
    Ok(report)
}

/// The service's background loop: relay rounds forever, sleeping `poll` between them. Runs on a dedicated
/// worker (a may coroutine — the sleeps are coroutine sleeps); it returns only on a database failure
/// (publish failures are recorded in the rows, not fatal).
pub fn run_relay<O: OutboxTable, E: LifeExecutor, P: Publisher + ?Sized>(
    project: &str,
    service: &str,
    exec: &E,
    publisher: &P,
    poll: Duration,
) -> Result<(), ApiError> {
    loop {
        relay_batch::<O, E, P>(exec, project, service, publisher)?;
        may::coroutine::sleep(poll);
    }
}

/// What one consumer round did.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ConsumerReport {
    pub pulled: usize,
    pub handled: usize,
    /// Already in `processed_events` — acked without calling the handler (AC-3).
    pub duplicates: usize,
    /// The handler failed: rolled back and nacked for redelivery/dead-letter (AC-4).
    pub failed: usize,
    /// Bodies that did not parse as an envelope: acked away — a malformed body is not a retryable state and
    /// must not wedge the subscription (`10-events.md`: consumers re-validate shape).
    pub undecodable: usize,
}

/// One consumer round: pull, and per message run `handle` and the `processed_events` marker in ONE guarded
/// transaction, ack after the commit, skip (ack) when the `event_id` was already processed, nack on error.
pub fn consume_batch<P: ProcessedTable, S: Subscriber + ?Sized, Src: TxSource, F>(
    subscription: &str,
    source: &Src,
    subscriber: &mut S,
    handle: &F,
) -> Result<ConsumerReport, ApiError>
where
    F: Fn(&Src::Tx, &Envelope<Value>) -> Result<(), ApiError>,
{
    let messages = subscriber
        .pull(subscription, EVENT_BATCH_SIZE)
        .map_err(event_db_error)?;
    let mut report = ConsumerReport::default();
    for message in messages {
        report.pulled += 1;
        match consume_one::<P, S, Src, F>(subscription, source, subscriber, &message, handle)? {
            Outcome::Handled => report.handled += 1,
            Outcome::Duplicate => report.duplicates += 1,
            Outcome::Failed => report.failed += 1,
            Outcome::Undecodable => report.undecodable += 1,
        }
    }
    Ok(report)
}

/// How one pulled message ended (each maps to a [`ConsumerReport`] counter).
enum Outcome {
    Handled,
    Duplicate,
    Failed,
    Undecodable,
}

/// One message: parse (poison bodies are acked away), then [`deliver`] with ack-after-commit or
/// nack-after-rollback. A transport failure on the ack/nack itself is the only Err here.
fn consume_one<P: ProcessedTable, S: Subscriber + ?Sized, Src: TxSource, F>(
    subscription: &str,
    source: &Src,
    subscriber: &mut S,
    message: &InboundMessage,
    handle: &F,
) -> Result<Outcome, ApiError>
where
    F: Fn(&Src::Tx, &Envelope<Value>) -> Result<(), ApiError>,
{
    let ids = std::slice::from_ref(&message.ack_id);
    if let Ok(envelope) = serde_json::from_slice::<Envelope<Value>>(&message.data) {
        match deliver::<P, Src, F>(source, &envelope, handle) {
            Ok(Delivery::Duplicate) => {
                subscriber.ack(subscription, ids).map_err(event_db_error)?;
                Ok(Outcome::Duplicate)
            }
            Ok(Delivery::Handled) => {
                subscriber.ack(subscription, ids).map_err(event_db_error)?;
                Ok(Outcome::Handled)
            }
            Err(_) => {
                subscriber.nack(subscription, ids).map_err(event_db_error)?;
                Ok(Outcome::Failed)
            }
        }
    } else {
        subscriber.ack(subscription, ids).map_err(event_db_error)?;
        Ok(Outcome::Undecodable)
    }
}

/// The outcome of attempting one message inside its own transaction.
enum Delivery {
    Handled,
    Duplicate,
}

/// The single-message core of [`consume_batch`]: guard, dedupe, `handle`, marker, commit — one transaction.
/// An error means "already rolled back, nack this message" (AC-4); it is never a domain answer.
fn deliver<P: ProcessedTable, Src: TxSource, F>(
    source: &Src,
    envelope: &Envelope<Value>,
    handle: &F,
) -> Result<Delivery, ApiError>
where
    F: Fn(&Src::Tx, &Envelope<Value>) -> Result<(), ApiError>,
{
    let tx = source.begin_crm_tx()?;
    let outcome = (|| -> Result<Delivery, ApiError> {
        tx.apply_workspace_guard(envelope.workspace_id)?;
        if P::is_processed(&tx, envelope.event_id)? {
            return Ok(Delivery::Duplicate);
        }
        handle(&tx, envelope)?;
        P::record(&tx, envelope.event_id, &envelope.event_type)?;
        Ok(Delivery::Handled)
    })();
    match outcome {
        Err(error) => {
            tx.rollback();
            Err(error)
        }
        Ok(Delivery::Duplicate) => {
            tx.rollback();
            Ok(Delivery::Duplicate)
        }
        Ok(Delivery::Handled) => tx.commit().map(|()| Delivery::Handled),
    }
}

/// The service's background loop: consume rounds forever, sleeping `poll` after an empty pull.
pub fn run_consumer<P: ProcessedTable, S: Subscriber + ?Sized, Src: TxSource, F>(
    subscription: &str,
    source: &Src,
    subscriber: &mut S,
    handle: &F,
    poll: Duration,
) -> Result<(), ApiError>
where
    F: Fn(&Src::Tx, &Envelope<Value>) -> Result<(), ApiError>,
{
    loop {
        let report = consume_batch::<P, S, Src, F>(subscription, source, subscriber, handle)?;
        if report.pulled == 0 {
            may::coroutine::sleep(poll);
        }
    }
}

/// Records every published message in memory — the story's test publisher. Clone shares the log.
#[derive(Clone, Default)]
pub struct MemoryPublisher {
    published: Arc<Mutex<Vec<PublishedMessage>>>,
    failing: Arc<AtomicBool>,
}

/// One message a [`MemoryPublisher`] recorded.
#[derive(Clone, Debug)]
pub struct PublishedMessage {
    pub topic: String,
    pub body: Vec<u8>,
    pub attributes: Vec<(String, String)>,
    pub ordering_key: String,
}

impl MemoryPublisher {
    #[must_use]
    pub fn messages(&self) -> Vec<PublishedMessage> {
        shared(&self.published).clone()
    }

    /// Make every publish fail (the relay-failure test hook); `false` restores success.
    pub fn set_failing(&self, failing: bool) {
        self.failing.store(failing, Ordering::SeqCst);
    }
}

impl Publisher for MemoryPublisher {
    fn publish(
        &self,
        topic: &str,
        data: &[u8],
        attributes: &[(&str, &str)],
        ordering_key: &str,
    ) -> Result<(), String> {
        if self.failing.load(Ordering::SeqCst) {
            return Err("memory publisher is set to fail".to_string());
        }
        shared(&self.published).push(PublishedMessage {
            topic: topic.to_string(),
            body: data.to_vec(),
            attributes: attributes
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            ordering_key: ordering_key.to_string(),
        });
        Ok(())
    }
}

/// In-memory [`Subscriber`]: seeded with messages, records acks and nacks — the story's test transport.
#[derive(Default)]
pub struct MemorySubscriber {
    inbox: VecDeque<InboundMessage>,
    acked: Vec<String>,
    nacked: Vec<String>,
}

impl MemorySubscriber {
    pub fn push(&mut self, ack_id: &str, body: &[u8]) {
        self.inbox.push_back(InboundMessage {
            ack_id: ack_id.to_string(),
            data: body.to_vec(),
            delivery_attempt: 1,
        });
    }

    #[must_use]
    pub fn acked(&self) -> &[String] {
        &self.acked
    }

    #[must_use]
    pub fn nacked(&self) -> &[String] {
        &self.nacked
    }
}

impl Subscriber for MemorySubscriber {
    fn pull(&mut self, _subscription: &str, max: u32) -> Result<Vec<InboundMessage>, String> {
        let take = usize::try_from(max)
            .unwrap_or(usize::MAX)
            .min(self.inbox.len());
        Ok(self.inbox.drain(..take).collect())
    }

    fn ack(&mut self, _subscription: &str, ack_ids: &[String]) -> Result<(), String> {
        self.acked.extend(ack_ids.iter().cloned());
        Ok(())
    }

    fn nack(&mut self, _subscription: &str, ack_ids: &[String]) -> Result<(), String> {
        self.nacked.extend(ack_ids.iter().cloned());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Role;
    use lifeguard::may_postgres::{types::ToSql, Row};
    use lifeguard::LifeError;
    use std::collections::{HashMap, HashSet};
    use std::sync::LazyLock;

    /// The fake tables are process-global because the table traits are static-method (that is how a
    /// Lifeguard entity works); tests share one lock and reset them.
    static FAKE_LOCK: Mutex<()> = Mutex::new(());
    static OUTBOX_ROWS: LazyLock<Mutex<Vec<OutboxRow>>> = LazyLock::new(|| Mutex::new(Vec::new()));
    static OUTBOX_MARKS: LazyLock<Mutex<Vec<Uuid>>> = LazyLock::new(|| Mutex::new(Vec::new()));
    static OUTBOX_FAILURES: LazyLock<Mutex<Vec<(Uuid, String)>>> =
        LazyLock::new(|| Mutex::new(Vec::new()));
    static PROCESSED: LazyLock<Mutex<HashSet<Uuid>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

    struct FakeOutbox;
    struct FakeProcessed;

    impl OutboxTable for FakeOutbox {
        const PRODUCER: &'static str = "crm/audience";

        fn insert<E: LifeExecutor>(_exec: &E, row: &OutboxRow) -> Result<(), ApiError> {
            shared(&OUTBOX_ROWS).push(row.clone());
            Ok(())
        }

        fn unpublished<E: LifeExecutor>(_exec: &E, limit: u64) -> Result<Vec<OutboxRow>, ApiError> {
            let mut rows = shared(&OUTBOX_ROWS);
            rows.sort_by_key(|row| row.occurred_at);
            let taken: Vec<OutboxRow> = rows
                .iter()
                .take(usize::try_from(limit).unwrap_or(usize::MAX))
                .cloned()
                .collect();
            Ok(taken)
        }

        fn mark_published<E: LifeExecutor>(
            _exec: &E,
            id: Uuid,
            _at: DateTime<Utc>,
        ) -> Result<(), ApiError> {
            if let Some(row) = shared(&OUTBOX_ROWS).iter_mut().find(|row| row.id == id) {
                row.published_at = Some(Utc::now());
            }
            shared(&OUTBOX_MARKS).push(id);
            Ok(())
        }

        fn record_failure<E: LifeExecutor>(
            _exec: &E,
            id: Uuid,
            reason: &str,
        ) -> Result<(), ApiError> {
            if let Some(row) = shared(&OUTBOX_ROWS).iter_mut().find(|row| row.id == id) {
                row.attempts += 1;
                row.last_error = Some(reason.to_string());
            }
            shared(&OUTBOX_FAILURES).push((id, reason.to_string()));
            Ok(())
        }
    }

    impl ProcessedTable for FakeProcessed {
        fn is_processed<E: LifeExecutor>(_exec: &E, event_id: Uuid) -> Result<bool, ApiError> {
            Ok(shared(&PROCESSED).contains(&event_id))
        }

        fn record<E: LifeExecutor>(
            _exec: &E,
            event_id: Uuid,
            _event_type: &str,
        ) -> Result<(), ApiError> {
            shared(&PROCESSED).insert(event_id);
            Ok(())
        }
    }

    /// A no-op executor: the fakes never touch it, and calling it is a test bug.
    struct NoopExec;

    impl LifeExecutor for NoopExec {
        fn execute(&self, _query: &str, _params: &[&dyn ToSql]) -> Result<u64, LifeError> {
            Err(LifeError::Other(
                "the fake executor must not touch the database".to_string(),
            ))
        }

        fn query_one(&self, _query: &str, _params: &[&dyn ToSql]) -> Result<Row, LifeError> {
            Err(LifeError::Other(
                "the fake executor must not touch the database".to_string(),
            ))
        }

        fn query_all(&self, _query: &str, _params: &[&dyn ToSql]) -> Result<Vec<Row>, LifeError> {
            Err(LifeError::Other(
                "the fake executor must not touch the database".to_string(),
            ))
        }
    }

    /// The consumer's fake transaction: records what the machinery asked it to do.
    #[derive(Clone)]
    struct FakeTx {
        log: Arc<Mutex<Vec<String>>>,
        commit_hook: Option<Arc<dyn Fn() -> Result<(), ApiError> + Send + Sync>>,
    }

    impl LifeExecutor for FakeTx {
        fn execute(&self, _query: &str, _params: &[&dyn ToSql]) -> Result<u64, LifeError> {
            Err(LifeError::Other(
                "the fake executor must not touch the database".to_string(),
            ))
        }

        fn query_one(&self, _query: &str, _params: &[&dyn ToSql]) -> Result<Row, LifeError> {
            Err(LifeError::Other(
                "the fake executor must not touch the database".to_string(),
            ))
        }

        fn query_all(&self, _query: &str, _params: &[&dyn ToSql]) -> Result<Vec<Row>, LifeError> {
            Err(LifeError::Other(
                "the fake executor must not touch the database".to_string(),
            ))
        }
    }

    impl CrmTx for FakeTx {
        fn apply_workspace_guard(&self, workspace_id: Uuid) -> Result<(), ApiError> {
            shared(&self.log).push(format!("guard:{workspace_id}"));
            Ok(())
        }

        fn commit(self) -> Result<(), ApiError> {
            if let Some(hook) = &self.commit_hook {
                hook()?;
            }
            shared(&self.log).push("commit".to_string());
            Ok(())
        }

        fn rollback(self) {
            shared(&self.log).push("rollback".to_string());
        }
    }

    #[derive(Clone)]
    struct FakeSource {
        log: Arc<Mutex<Vec<String>>>,
        fail_commit: Arc<AtomicBool>,
    }

    impl FakeSource {
        fn new() -> Self {
            FakeSource {
                log: Arc::new(Mutex::new(Vec::new())),
                fail_commit: Arc::new(AtomicBool::new(false)),
            }
        }

        fn failing_commit(&self) -> FakeSource {
            self.fail_commit.store(true, Ordering::SeqCst);
            self.clone()
        }
    }

    impl TxSource for FakeSource {
        type Tx = FakeTx;

        fn begin_crm_tx(&self) -> Result<Self::Tx, ApiError> {
            let hook: Option<Arc<dyn Fn() -> Result<(), ApiError> + Send + Sync>> =
                self.fail_commit.load(Ordering::SeqCst).then(|| {
                    let flag = Arc::clone(&self.fail_commit);
                    let hook: Arc<dyn Fn() -> Result<(), ApiError> + Send + Sync> =
                        Arc::new(move || {
                            if flag.load(Ordering::SeqCst) {
                                Err(event_db_error("fake commit failed"))
                            } else {
                                Ok(())
                            }
                        });
                    hook
                });
            Ok(FakeTx {
                log: Arc::clone(&self.log),
                commit_hook: hook,
            })
        }
    }

    fn ctx() -> RequestContext {
        RequestContext {
            workspace_id: Uuid::now_v7(),
            actor_kind: ActorKind::User,
            actor_id: "user-1".to_string(),
            role: Role::Admin,
            request_id: Some("req-1".to_string()),
        }
    }

    fn reset_fakes() {
        shared(&OUTBOX_ROWS).clear();
        shared(&OUTBOX_MARKS).clear();
        shared(&OUTBOX_FAILURES).clear();
        shared(&PROCESSED).clear();
    }

    fn seed_row(event_type: &str) -> OutboxRow {
        let envelope = build_envelope::<FakeOutbox, _>(
            &ctx(),
            event_type,
            "entity-1",
            &serde_json::json!({"k": 1}),
        )
        .expect("build envelope");
        let row = OutboxRow {
            id: envelope.event_id,
            workspace_id: envelope.workspace_id,
            event_type: envelope.event_type.clone(),
            entity_id: envelope.entity_id.clone(),
            payload: serde_json::to_value(&envelope).expect("serialize envelope"),
            occurred_at: envelope.occurred_at,
            published_at: None,
            attempts: 0,
            last_error: None,
        };
        FakeOutbox::insert(&NoopExec, &row).expect("seed fake outbox");
        row
    }

    #[test]
    fn relay_publishes_with_contract_attributes_and_marks_rows() {
        let _serial = shared(&FAKE_LOCK);
        reset_fakes();
        let row = seed_row("contact.created");
        let publisher = MemoryPublisher::default();

        let report = relay_batch::<FakeOutbox, NoopExec, MemoryPublisher>(
            &NoopExec,
            "rerp-dev-247417",
            "audience",
            &publisher,
        )
        .expect("relay round");

        assert_eq!(report.published, 1);
        assert_eq!(report.failed, 0);
        let message = &publisher.messages()[0];
        assert_eq!(message.topic, topic_for("rerp-dev-247417", "audience"));
        assert_eq!(message.ordering_key, "entity-1");
        let mut attrs: HashMap<String, String> = message.attributes.iter().cloned().collect();
        assert_eq!(
            attrs.remove("event_type").as_deref(),
            Some("contact.created")
        );
        assert_eq!(
            attrs.remove("workspace_id").as_deref(),
            Some(row.workspace_id.to_string().as_str())
        );
        assert_eq!(attrs.remove("schema_version").as_deref(), Some("1"));
        assert!(attrs.is_empty());
        let body: Value = serde_json::from_slice(&message.body).expect("body is JSON");
        assert_eq!(body["event_id"].as_str(), Some(row.id.to_string().as_str()));

        // The row is marked published; a second round has nothing left (AC-1 ordering half).
        assert_eq!(shared(&OUTBOX_MARKS).as_slice(), &[row.id]);
        let again = relay_batch::<FakeOutbox, NoopExec, MemoryPublisher>(
            &NoopExec, "p", "audience", &publisher,
        )
        .expect("second relay round");
        // The fake read ignores published_at like nothing else would; verify the mark, not a re-read.
        assert_eq!(
            again.published + again.failed,
            publisher.messages().len() - 1
        );
        assert!(shared(&OUTBOX_ROWS)
            .iter()
            .all(|r| r.published_at.is_some()));
    }

    #[test]
    fn relay_records_failures_and_retries_next_round() {
        let _serial = shared(&FAKE_LOCK);
        reset_fakes();
        let row = seed_row("contact.updated");
        let publisher = MemoryPublisher::default();
        publisher.set_failing(true);

        let report = relay_batch::<FakeOutbox, NoopExec, MemoryPublisher>(
            &NoopExec, "p", "audience", &publisher,
        )
        .expect("relay round with failing publisher");
        assert_eq!(report.failed, 1);
        assert_eq!(report.published, 0);

        let failures = shared(&OUTBOX_FAILURES).clone();
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].0, row.id);
        assert!(failures[0]
            .1
            .contains("publish to projects/p/topics/crm.audience.v1"));
        let (published_at, attempts) = {
            let rows = shared(&OUTBOX_ROWS);
            (rows[0].published_at, rows[0].attempts)
        };
        assert!(published_at.is_none());
        assert_eq!(attempts, 1);

        // Next round retries the same row (at-least-once).
        publisher.set_failing(false);
        let retry = relay_batch::<FakeOutbox, NoopExec, MemoryPublisher>(
            &NoopExec, "p", "audience", &publisher,
        )
        .expect("retry round");
        assert_eq!(retry.published, 1);
        assert_eq!(publisher.messages().len(), 1);
    }

    #[test]
    fn consumer_handles_once_and_acks_duplicates_without_recalling_the_handler() {
        let _serial = shared(&FAKE_LOCK);
        reset_fakes();
        let envelope = build_envelope::<FakeOutbox, _>(
            &ctx(),
            "contact.created",
            "e-1",
            &serde_json::json!({}),
        )
        .expect("envelope");
        let body = serde_json::to_vec(&envelope).expect("body");

        let source = FakeSource::new();
        let mut subscriber = MemorySubscriber::default();
        subscriber.push("ack-1", &body);
        subscriber.push("ack-2", &body);
        let calls = Arc::new(AtomicBool::new(false));
        let calls_ref = Arc::clone(&calls);
        let handle = |_tx: &FakeTx, _env: &Envelope<Value>| {
            assert!(
                !calls_ref.swap(true, Ordering::SeqCst),
                "handler must run once per event_id"
            );
            Ok(())
        };

        let report = consume_batch::<FakeProcessed, MemorySubscriber, FakeSource, _>(
            "projects/p/subscriptions/s",
            &source,
            &mut subscriber,
            &handle,
        )
        .expect("consume round");

        assert_eq!(report.pulled, 2);
        assert_eq!(report.handled, 1);
        assert_eq!(report.duplicates, 1);
        assert_eq!(
            subscriber.acked(),
            &["ack-1".to_string(), "ack-2".to_string()]
        );
        assert_eq!(subscriber.nacked(), &[] as &[String]);
        let log = shared(&source.log).clone();
        assert_eq!(
            log.iter()
                .filter(|entry| *entry == &format!("guard:{}", envelope.workspace_id))
                .count(),
            2
        );
        assert_eq!(log.iter().filter(|entry| *entry == "commit").count(), 1);
    }

    #[test]
    fn consumer_rolls_back_and_nacks_when_the_handler_fails() {
        let _serial = shared(&FAKE_LOCK);
        reset_fakes();
        let envelope = build_envelope::<FakeOutbox, _>(
            &ctx(),
            "contact.created",
            "e-1",
            &serde_json::json!({}),
        )
        .expect("envelope");
        let body = serde_json::to_vec(&envelope).expect("body");

        let source = FakeSource::new();
        let mut subscriber = MemorySubscriber::default();
        subscriber.push("ack-1", &body);
        let handle =
            |_tx: &FakeTx, _env: &Envelope<Value>| Err(ApiError::forbidden("handler failure"));

        let report = consume_batch::<FakeProcessed, MemorySubscriber, FakeSource, _>(
            "sub",
            &source,
            &mut subscriber,
            &handle,
        )
        .expect("consume round");

        assert_eq!(report.failed, 1);
        assert_eq!(subscriber.nacked(), &["ack-1".to_string()]);
        assert_eq!(subscriber.acked(), &[] as &[String]);
        assert!(shared(&source.log).contains(&"rollback".to_string()));
        assert!(!shared(&PROCESSED).contains(&envelope.event_id));
    }

    #[test]
    fn consumer_acks_poison_bodies_away() {
        let _serial = shared(&FAKE_LOCK);
        reset_fakes();
        let source = FakeSource::new();
        let mut subscriber = MemorySubscriber::default();
        subscriber.push("ack-1", b"this is not an envelope");

        let report = consume_batch::<FakeProcessed, MemorySubscriber, FakeSource, _>(
            "sub",
            &source,
            &mut subscriber,
            &|_tx: &FakeTx, _env: &Envelope<Value>| Ok(()),
        )
        .expect("consume round");

        assert_eq!(report.undecodable, 1);
        assert_eq!(subscriber.acked(), &["ack-1".to_string()]);
        assert_eq!(subscriber.nacked(), &[] as &[String]);
    }

    #[test]
    fn consumer_nacks_when_the_commit_itself_fails() {
        let _serial = shared(&FAKE_LOCK);
        reset_fakes();
        let envelope = build_envelope::<FakeOutbox, _>(
            &ctx(),
            "contact.created",
            "e-1",
            &serde_json::json!({}),
        )
        .expect("envelope");
        let body = serde_json::to_vec(&envelope).expect("body");

        let source = FakeSource::new().failing_commit();
        let mut subscriber = MemorySubscriber::default();
        subscriber.push("ack-1", &body);
        let handle = |_tx: &FakeTx, _env: &Envelope<Value>| Ok(());

        let report = consume_batch::<FakeProcessed, MemorySubscriber, FakeSource, _>(
            "sub",
            &source,
            &mut subscriber,
            &handle,
        )
        .expect("consume round");

        // A commit that fails must never be acked: the marker did not land, so redelivery must happen.
        assert_eq!(report.failed, 1);
        assert_eq!(subscriber.nacked(), &["ack-1".to_string()]);
        assert_eq!(subscriber.acked(), &[] as &[String]);
    }

    #[test]
    fn envelope_fields_match_the_generated_asyncapi_payload_contracts() {
        let envelope = build_envelope::<FakeOutbox, _>(
            &ctx(),
            "contact.created",
            "e-1",
            &serde_json::json!({"x": 1}),
        )
        .expect("envelope");
        let value = serde_json::to_value(&envelope).expect("serialized envelope");
        let ours: HashSet<String> = value.as_object().expect("object").keys().cloned().collect();
        let contract: HashSet<String> = [
            "event_id",
            "event_type",
            "workspace_id",
            "occurred_at",
            "producer",
            "actor",
            "request_id",
            "entity_id",
            "data",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        assert_eq!(ours, contract);
        assert_eq!(envelope.actor.kind, "USER");
        assert_eq!(envelope.producer, "crm/audience");
        assert_eq!(
            envelope.event_id.as_bytes()[6] >> 4,
            7,
            "event_id must be UUID v7"
        );
        assert_eq!(value["request_id"], serde_json::json!("req-1"));

        // AC-2: every audience message's inline payload carries exactly these property names, and requires
        // only the always-present subset (request_id/actor/entity_id are optional in the contract).
        let doc_path = format!(
            "{}/../../../openapi/crm/audience/asyncapi.yaml",
            env!("CARGO_MANIFEST_DIR")
        );
        let doc: serde_yaml::Value = serde_yaml::from_str(
            &std::fs::read_to_string(&doc_path).expect("asyncapi.yaml readable"),
        )
        .expect("asyncapi.yaml parses");
        let messages = doc["components"]["messages"]
            .as_mapping()
            .expect("components.messages mapping");
        assert!(!messages.is_empty());
        for (key, message) in messages {
            let name = key.as_str().unwrap_or("message");
            let payload = &message["payload"];
            let properties: HashSet<String> = payload["properties"]
                .as_mapping()
                .unwrap_or_else(|| panic!("{name}: inline payload properties"))
                .keys()
                .map(|key| key.as_str().expect("string key").to_string())
                .collect();
            assert_eq!(
                properties, contract,
                "{name}: payload properties are the envelope contract"
            );
            let required: HashSet<String> = payload["required"]
                .as_sequence()
                .unwrap_or_else(|| panic!("{name}: payload required list"))
                .iter()
                .map(|item| item.as_str().expect("string item").to_string())
                .collect();
            assert!(
                required.is_subset(&contract),
                "{name}: required beyond the envelope fields"
            );
            assert!(required.contains("event_id") && required.contains("data"));
            assert!(
                !required.contains("request_id"),
                "{name}: request_id must stay optional"
            );
        }
    }

    #[test]
    fn append_builds_the_row_the_relay_will_later_publish() {
        let _serial = shared(&FAKE_LOCK);
        reset_fakes();
        let id = append::<FakeOutbox, _, NoopExec>(
            &NoopExec,
            &ctx(),
            "contact.created",
            "e-9",
            &serde_json::json!({"email": "a@b.c"}),
        )
        .expect("append");
        let row = shared(&OUTBOX_ROWS)[0].clone();
        assert_eq!(row.id, id);
        assert_eq!(row.entity_id, "e-9");
        assert!(row.published_at.is_none());
        assert_eq!(row.attempts, 0);
        assert_eq!(
            row.payload["event_type"],
            serde_json::json!("contact.created")
        );
        assert_eq!(row.payload["request_id"], serde_json::json!("req-1"));
    }
}
