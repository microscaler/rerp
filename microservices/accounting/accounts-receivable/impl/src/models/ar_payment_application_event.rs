//! Audit journal for payment-application transitions (Epic F2b/F2c).
//!
//! The `ar_payment_applications` pair row (UNIQUE per payment+invoice) is
//! the CURRENT state; every applied delta and (F2c) reversal is journaled
//! here, so "multiple applications to the same document" and allocation
//! reversal both have immutable audit linkage — the contract's answer to
//! the pair-row collapse.

use lifeguard_derive::{LifeModel, LifeRecord};

#[derive(Debug, Clone, LifeModel, LifeRecord)]
#[table_name = "ar_payment_application_events"]
#[table_comment = "Immutable audit journal of payment-application transitions"]
#[composite_unique = "tenant_id, legal_entity_id, id"]
#[index = "idx_ar_payment_application_events_scope(tenant_id, legal_entity_id)"]
#[index = "idx_ar_payment_application_events_application(application_id)"]
pub struct ArPaymentApplicationEvent {
    #[primary_key]
    pub id: uuid::Uuid,

    #[indexed]
    #[column_type = "VARCHAR(200)"]
    pub tenant_id: String,

    #[indexed]
    pub legal_entity_id: uuid::Uuid,

    /// The application pair row this event transitioned.
    #[indexed]
    pub application_id: uuid::Uuid,

    /// APPLIED or REVERSED (F2c).
    #[column_type = "VARCHAR(20)"]
    pub kind: String,

    /// Positive magnitude; the sign is carried by `kind`.
    #[column_type = "NUMERIC(19, 6)"]
    pub amount: rust_decimal::Decimal,

    pub actor_id: Option<uuid::Uuid>,
    pub occurred_at: chrono::NaiveDateTime,

    /// Reversal reason (F2c); null on applied entries.
    #[column_type = "VARCHAR(1000)"]
    pub reason: Option<String>,
}
