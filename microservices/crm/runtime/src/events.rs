//! The event envelope (`event_id`, `event_type`, `workspace_id`, `occurred_at`, `producer`, `actor`, `request_id`, `entity_id`, `data`), the outbox writer (same transaction as the change), the relay (publish after commit, Pub/Sub attributes `event_type` and `workspace_id`, ordering key `entity_id`), and the idempotent consumer (`processed_events`, ack after commit, nack to retry). openapi/crm/docs/10-events.md.
//!
//! Story: CRM-RT-02 (docs/crm/stories/).
