//! The CRM event machinery against the real Lifeguard tables — these tests own real entities over
//! `audience.outbox` / `audience.processed_events`, exactly what a service's `impl/src/models/` will
//! implement. Prepare the database with `microservices/crm/scripts/crm-db.sh up` (plus the service's
//! migrations applying `migrations/templates/rls_and_infrastructure.sql`), then:
//!
//! ```text
//! CRM_TEST_DATABASE_URL='postgres://…@127.0.0.1:6543/crm' \
//!   cargo test -p rerp_crm_runtime --test events_db -- --ignored
//! ```

#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    clippy::disallowed_methods,
    reason = "integration test harness: helpers fail the test by panicking, and ordering gaps use real thread sleeps on OS test threads"
)]
#![allow(
    dead_code,
    reason = "the LifeModel/LifeRecord derives consume these structs; their generated Model/Record/Entity/Column types are what the code constructs"
)]

use std::sync::{Mutex, PoisonError};

use chrono::{DateTime, Utc};
use lifeguard::active_model::ActiveModelTrait;
use lifeguard::raw_sql::{execute_statement, query_value};
use lifeguard::transaction::Transaction;
use lifeguard::{connect, ColumnTrait, LifeExecutor, LifeModelTrait, MayPostgresExecutor, Order};
use lifeguard_derive::{LifeModel, LifeRecord};
use rerp_crm_runtime::context::RequestContext;
use rerp_crm_runtime::errors::ApiError;
use rerp_crm_runtime::events::{
    append, build_envelope, consume_batch, event_db_error, relay_batch, topic_for, CrmTx, Envelope,
    MemoryPublisher, MemorySubscriber, OutboxRow, OutboxTable, ProcessedTable, TxSource,
};
use serde_json::{json, Value};
use uuid::Uuid;

/// The harness tables are shared; the tests take turns (each resets them first anyway).
static DB_LOCK: Mutex<()> = Mutex::new(());

fn serial() -> std::sync::MutexGuard<'static, ()> {
    DB_LOCK.lock().unwrap_or_else(PoisonError::into_inner)
}

/// The service's `<schema>.outbox` entity (the shape `rls_and_infrastructure.sql` provisions).
mod outbox {
    use super::*;

    #[derive(Clone, Debug, LifeModel, LifeRecord)]
    #[table_name = "outbox"]
    #[schema_name = "audience"]
    pub struct OutboxRowEntity {
        #[primary_key]
        #[column_type = "UUID"]
        pub id: Uuid,
        #[column_type = "UUID"]
        pub workspace_id: Uuid,
        pub event_type: String,
        pub entity_id: String,
        #[column_type = "JSONB"]
        pub payload: Value,
        pub occurred_at: DateTime<Utc>,
        #[nullable]
        pub published_at: Option<DateTime<Utc>>,
        #[column_type = "INTEGER"]
        pub attempts: i32,
        #[nullable]
        pub last_error: Option<String>,
    }

    impl OutboxTable for Entity {
        const PRODUCER: &'static str = "crm/audience";

        fn insert<E: LifeExecutor>(exec: &E, row: &OutboxRow) -> Result<(), ApiError> {
            OutboxRowEntityRecord::new()
                .set_id(row.id)
                .set_workspace_id(row.workspace_id)
                .set_event_type(row.event_type.clone())
                .set_entity_id(row.entity_id.clone())
                .set_payload(row.payload.clone())
                .set_occurred_at(row.occurred_at)
                .set_published_at(None)
                .set_attempts(0)
                .set_last_error(None)
                .insert(exec)
                .map(|_| ())
                .map_err(event_db_error)
        }

        fn unpublished<E: LifeExecutor>(exec: &E, limit: u64) -> Result<Vec<OutboxRow>, ApiError> {
            let models = Entity::find()
                .filter(Column::PublishedAt.is_null())
                .order_by(Column::OccurredAt, Order::Asc)
                .limit(limit)
                .all(exec)
                .map_err(event_db_error)?;
            Ok(models
                .into_iter()
                .map(|model| OutboxRow {
                    id: model.id,
                    workspace_id: model.workspace_id,
                    event_type: model.event_type,
                    entity_id: model.entity_id,
                    payload: model.payload,
                    occurred_at: model.occurred_at,
                    published_at: model.published_at,
                    attempts: model.attempts,
                    last_error: model.last_error,
                })
                .collect())
        }

        fn mark_published<E: LifeExecutor>(
            exec: &E,
            id: Uuid,
            published_at: DateTime<Utc>,
        ) -> Result<(), ApiError> {
            let model = Self::must_find(exec, id)?;
            OutboxRowEntityRecord::new()
                .set_id(model.id)
                .set_workspace_id(model.workspace_id)
                .set_event_type(model.event_type)
                .set_entity_id(model.entity_id)
                .set_payload(model.payload)
                .set_occurred_at(model.occurred_at)
                .set_published_at(Some(published_at))
                .set_attempts(model.attempts)
                .set_last_error(model.last_error)
                .update(exec)
                .map(|_| ())
                .map_err(event_db_error)
        }

        fn record_failure<E: LifeExecutor>(
            exec: &E,
            id: Uuid,
            last_error: &str,
        ) -> Result<(), ApiError> {
            let model = Self::must_find(exec, id)?;
            OutboxRowEntityRecord::new()
                .set_id(model.id)
                .set_workspace_id(model.workspace_id)
                .set_event_type(model.event_type)
                .set_entity_id(model.entity_id)
                .set_payload(model.payload)
                .set_occurred_at(model.occurred_at)
                .set_published_at(model.published_at)
                .set_attempts(model.attempts + 1)
                .set_last_error(Some(last_error.to_string()))
                .update(exec)
                .map(|_| ())
                .map_err(event_db_error)
        }
    }

    impl Entity {
        fn must_find<E: LifeExecutor>(
            exec: &E,
            id: Uuid,
        ) -> Result<OutboxRowEntityModel, ApiError> {
            Entity::find()
                .filter(Column::Id.eq(id))
                .find_one(exec)
                .map_err(event_db_error)?
                .ok_or_else(|| event_db_error(format!("outbox row {id} vanished")))
        }
    }
}

/// The service's `<schema>.processed_events` entity: the consumer's idempotency ledger.
mod processed {
    use super::*;

    #[derive(Clone, Debug, LifeModel, LifeRecord)]
    #[table_name = "processed_events"]
    #[schema_name = "audience"]
    pub struct ProcessedEvent {
        #[primary_key]
        #[column_type = "UUID"]
        pub event_id: Uuid,
        pub event_type: String,
        pub processed_at: DateTime<Utc>,
    }

    impl ProcessedTable for Entity {
        fn is_processed<E: LifeExecutor>(exec: &E, event_id: Uuid) -> Result<bool, ApiError> {
            Entity::find()
                .filter(Column::EventId.eq(event_id))
                .find_one(exec)
                .map(|found| found.is_some())
                .map_err(event_db_error)
        }

        fn record<E: LifeExecutor>(
            exec: &E,
            event_id: Uuid,
            event_type: &str,
        ) -> Result<(), ApiError> {
            ProcessedEventRecord::new()
                .set_event_id(event_id)
                .set_event_type(event_type.to_string())
                .set_processed_at(Utc::now())
                .insert(exec)
                .map(|_| ())
                .map_err(event_db_error)
        }
    }
}

/// A test's own executor: `MayPostgresExecutor` wraps a may coroutine client and is intentionally not
/// `Sync`, so each test builds one from `CRM_TEST_DATABASE_URL` on its own thread (the lifeguard
/// `TestDatabase` pattern).
fn test_db() -> MayPostgresExecutor {
    let url = std::env::var("CRM_TEST_DATABASE_URL")
        .expect("set CRM_TEST_DATABASE_URL to the prepared CRM database (see file header)");
    let client = connect(&url).expect("connect the CRM test database");
    MayPostgresExecutor::new(client)
}

fn reset_tables(db: &MayPostgresExecutor) {
    execute_statement(db, "DELETE FROM audience.outbox", &[]).expect("clear outbox");
    execute_statement(db, "DELETE FROM audience.processed_events", &[])
        .expect("clear processed_events");
}

fn count_rows<E: LifeExecutor>(exec: &E, sql: &str) -> i64 {
    query_value::<i64, _>(exec, sql, &[]).expect("count query")
}

fn outbox_total(db: &MayPostgresExecutor) -> i64 {
    count_rows(db, "SELECT count(*) FROM audience.outbox")
}

fn processed_total(db: &MayPostgresExecutor) -> i64 {
    count_rows(db, "SELECT count(*) FROM audience.processed_events")
}

fn ctx(workspace_id: Uuid) -> RequestContext {
    RequestContext::system(workspace_id, "events-db-test")
}

/// Commit one appended event the way a service request does: guard, append, commit.
fn commit_one_event(
    db: &MayPostgresExecutor,
    workspace_id: Uuid,
    event_type: &str,
    entity_id: &str,
) -> Uuid {
    let tx = db.begin_crm_tx().expect("begin tx");
    tx.apply_workspace_guard(workspace_id).expect("guard");
    let id =
        append::<outbox::Entity, _, _>(&tx, &ctx(workspace_id), event_type, entity_id, &json!({}))
            .expect("append");
    tx.commit().expect("commit");
    id
}

/// AC-1 (transaction half): an append that rolls back with its transaction leaves no row; the committed
/// append is there for the relay.
#[test]
#[ignore = "requires CRM_TEST_DATABASE_URL (see file header)"]
fn append_rolls_back_with_the_transaction_and_a_committed_append_survives() {
    let _serial = serial();
    let db = test_db();
    reset_tables(&db);
    let workspace_id = Uuid::now_v7();

    let tx = db.begin_crm_tx().expect("begin tx");
    tx.apply_workspace_guard(workspace_id).expect("guard");
    append::<outbox::Entity, _, _>(
        &tx,
        &ctx(workspace_id),
        "contact.created",
        "gone",
        &json!({"rolled": "back"}),
    )
    .expect("append inside a transaction that will roll back");
    Transaction::rollback(tx).expect("rollback");
    assert_eq!(outbox_total(&db), 0, "a rollback leaves no event (AC-1)");

    let kept = commit_one_event(&db, workspace_id, "contact.created", "kept");
    assert_eq!(outbox_total(&db), 1);
    let found = count_rows(
        &db,
        &format!(
            "SELECT count(*) FROM audience.outbox WHERE id = '{kept}' AND payload->>'entity_id' = 'kept'"
        ),
    );
    assert_eq!(
        found, 1,
        "the committed row is the appended one, envelope payload intact"
    );
}

/// AC-1 + attributes: committed rows publish oldest-first to the service topic with the contract
/// attributes and the entity as ordering key, get marked, and a second round republishes nothing.
#[test]
#[ignore = "requires CRM_TEST_DATABASE_URL (see file header)"]
fn relay_publishes_committed_rows_oldest_first_and_never_republishes_them() {
    let _serial = serial();
    let db = test_db();
    reset_tables(&db);
    let workspace_id = Uuid::now_v7();

    let older = commit_one_event(&db, workspace_id, "contact.created", "e-old");
    std::thread::sleep(std::time::Duration::from_millis(5));
    commit_one_event(&db, workspace_id, "contact.updated", "e-new");

    let publisher = MemoryPublisher::default();
    let report = relay_batch::<outbox::Entity, MayPostgresExecutor, MemoryPublisher>(
        &db, "proj", "audience", &publisher,
    )
    .expect("relay round");
    assert_eq!(report.published, 2);

    let messages = publisher.messages();
    assert_eq!(messages.len(), 2);
    let first: Envelope<Value> = serde_json::from_slice(&messages[0].body).expect("envelope body");
    assert_eq!(first.event_id, older, "oldest occurred_at publishes first");
    assert_eq!(messages[0].topic, topic_for("proj", "audience"));
    assert_eq!(messages[1].ordering_key, "e-new");
    let attrs: std::collections::HashMap<String, String> =
        messages[0].attributes.iter().cloned().collect();
    assert_eq!(attrs["event_type"], "contact.created");
    assert_eq!(attrs["workspace_id"], workspace_id.to_string());
    assert_eq!(attrs["schema_version"], "1");

    assert_eq!(
        count_rows(
            &db,
            "SELECT count(*) FROM audience.outbox WHERE published_at IS NOT NULL"
        ),
        2
    );

    let again = relay_batch::<outbox::Entity, MayPostgresExecutor, MemoryPublisher>(
        &db, "proj", "audience", &publisher,
    )
    .expect("second round");
    assert_eq!(
        again.published + again.failed,
        0,
        "published rows are never republished"
    );
    assert_eq!(publisher.messages().len(), 2);
}

/// A failed publish keeps the row unpublished with the reason recorded, and the next round retries it.
#[test]
#[ignore = "requires CRM_TEST_DATABASE_URL (see file header)"]
fn relay_records_publish_failures_and_retries_them_next_round() {
    let _serial = serial();
    let db = test_db();
    reset_tables(&db);
    let workspace_id = Uuid::now_v7();
    commit_one_event(&db, workspace_id, "contact.created", "e-1");

    let publisher = MemoryPublisher::default();
    publisher.set_failing(true);
    let report = relay_batch::<outbox::Entity, MayPostgresExecutor, MemoryPublisher>(
        &db, "p", "audience", &publisher,
    )
    .expect("failing round");
    assert_eq!(report.failed, 1);
    assert_eq!(
        count_rows(
            &db,
            "SELECT count(*) FROM audience.outbox \
             WHERE published_at IS NULL AND attempts = 1 AND last_error IS NOT NULL"
        ),
        1,
        "the row waits unpublished with attempts counted and the reason kept"
    );

    publisher.set_failing(false);
    let retry = relay_batch::<outbox::Entity, MayPostgresExecutor, MemoryPublisher>(
        &db, "p", "audience", &publisher,
    )
    .expect("retry round");
    assert_eq!(retry.published, 1);
    assert_eq!(publisher.messages().len(), 1);
    assert_eq!(
        count_rows(
            &db,
            "SELECT count(*) FROM audience.outbox WHERE published_at IS NOT NULL"
        ),
        1
    );
}

/// AC-3 end to end: the same event delivered twice is handled exactly once, the real ledger holds one
/// marker, and the handler ran under the envelope's workspace guard.
#[test]
#[ignore = "requires CRM_TEST_DATABASE_URL (see file header)"]
fn consumer_handles_once_and_skips_the_duplicate_with_the_real_ledger() {
    let _serial = serial();
    let db = test_db();
    reset_tables(&db);
    let workspace_id = Uuid::now_v7();

    let envelope = build_envelope::<outbox::Entity, _>(
        &ctx(workspace_id),
        "contact.created",
        "e-1",
        &json!({"email": "dup@example.test"}),
    )
    .expect("envelope");
    let body = serde_json::to_vec(&envelope).expect("body");

    let mut subscriber = MemorySubscriber::default();
    subscriber.push("ack-1", &body);
    subscriber.push("ack-2", &body);

    let calls = std::sync::atomic::AtomicUsize::new(0);
    let handle = |tx: &Transaction, env: &Envelope<Value>| {
        let setting: Option<String> = query_value::<Option<String>, _>(
            tx,
            "SELECT current_setting('crm.workspace_id', true)",
            &[],
        )
        .expect("read the guard");
        assert_eq!(
            setting.as_deref(),
            Some(env.workspace_id.to_string().as_str()),
            "the guard is set for the handler"
        );
        calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    };

    let report = consume_batch::<processed::Entity, MemorySubscriber, MayPostgresExecutor, _>(
        "projects/p/subscriptions/s",
        &db,
        &mut subscriber,
        &handle,
    )
    .expect("consume round");

    assert_eq!(report.handled, 1);
    assert_eq!(report.duplicates, 1);
    assert_eq!(
        calls.load(std::sync::atomic::Ordering::SeqCst),
        1,
        "the handler runs once per event_id"
    );
    assert_eq!(subscriber.acked().len(), 2, "both deliveries are acked");
    assert_eq!(processed_total(&db), 1, "the ledger holds one marker");
}

/// AC-4 end to end: a failing handler rolls back BOTH its writes and the marker; the message is nacked.
#[test]
#[ignore = "requires CRM_TEST_DATABASE_URL (see file header)"]
fn consumer_rolls_back_handler_writes_and_the_marker_then_nacks() {
    let _serial = serial();
    let db = test_db();
    reset_tables(&db);
    let workspace_id = Uuid::now_v7();

    let envelope = build_envelope::<outbox::Entity, _>(
        &ctx(workspace_id),
        "segment.updated",
        "e-1",
        &json!({}),
    )
    .expect("envelope");
    let body = serde_json::to_vec(&envelope).expect("body");

    let mut subscriber = MemorySubscriber::default();
    subscriber.push("ack-1", &body);

    let handle = |tx: &Transaction, env: &Envelope<Value>| {
        append::<outbox::Entity, _, _>(
            tx,
            &ctx(env.workspace_id),
            "probe.rollback",
            "e-1",
            &json!({"side": "effect"}),
        )
        .expect("handler write inside the transaction");
        Err(ApiError::forbidden("the handler always fails"))
    };

    let report = consume_batch::<processed::Entity, MemorySubscriber, MayPostgresExecutor, _>(
        "projects/p/subscriptions/s",
        &db,
        &mut subscriber,
        &handle,
    )
    .expect("consume round");

    assert_eq!(report.failed, 1);
    assert_eq!(
        subscriber.nacked(),
        &["ack-1".to_string()],
        "redelivery is expected (dead-letter after retries)"
    );
    assert_eq!(
        outbox_total(&db),
        0,
        "the handler's write rolled back with the transaction"
    );
    assert_eq!(
        processed_total(&db),
        0,
        "no marker was committed for a failed handling"
    );
}

/// A body that is not an envelope is acked away, never nacked into a retry loop.
#[test]
#[ignore = "requires CRM_TEST_DATABASE_URL (see file header)"]
fn poison_bodies_are_acked_away() {
    let _serial = serial();
    let db = test_db();
    reset_tables(&db);
    let mut subscriber = MemorySubscriber::default();
    subscriber.push("ack-1", b"{ not json");

    let report = consume_batch::<processed::Entity, MemorySubscriber, MayPostgresExecutor, _>(
        "projects/p/subscriptions/s",
        &db,
        &mut subscriber,
        &|_tx: &Transaction, _env: &Envelope<Value>| Ok(()),
    )
    .expect("consume round");

    assert_eq!(report.undecodable, 1);
    assert_eq!(subscriber.acked(), &["ack-1".to_string()]);
    assert_eq!(subscriber.nacked(), &[] as &[String]);
    assert_eq!(processed_total(&db), 0);
}
