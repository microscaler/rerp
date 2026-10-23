//! The conversion intake (ADR 003): the only way a customer enters RERP.
//!
//! Pulls Pub/Sub `crm-conversions` from inside the cluster. Nothing outside dials in. Every message is untrusted:
//!
//!   1. `rerp_crm_conversion::screen`: size, schema with no unknown fields, field checks, and a valid signature by a
//!      key this service trusts that may sign that kind of conversion: the pipeline's for manual conversions,
//!      PriceWhisperer billing's for paid Stripe checkouts (billing has re-retrieved the checkout from Stripe; RERP
//!      holds no Stripe credential);
//!   2. the tenant must be one this intake serves;
//!   3. `accounts::record` writes it once: the conversion id is deterministic, so a replay changes nothing.
//!
//! Ack after the write commits, or when the message is not ours to process (logged and dropped). Database failures
//! nack, so Pub/Sub redelivers with back-off and dead-letters after its attempt limit.
//!
//! Env: CRM_CONVERSIONS_SUBSCRIPTION, CRM_CONVERSION_VERIFY_KEYS(_*), CRM_INTAKE_TENANTS.

use lifeguard::SessionContext;
use rerp_crm_conversion::pubsub::PubSub;
use rerp_crm_conversion::{screen, Conversion, Keys, Screened};
use std::time::Duration;
use uuid::Uuid;

use crate::accounts::{self, Recorded};

/// The intake's own subject in RLS sessions and audit columns.
const INTAKE_SUBJECT: Uuid = Uuid::from_u128(0x0c7a_11e5_6c1e_4d3a_9b8e_2f0a_7c5d_1e01);

#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Recorded now, or already recorded: ack.
    Done(String),
    /// Not ours to process (malformed, unsigned, foreign tenant, unknown to Stripe): ack and drop, logged.
    Drop(String),
    /// Transient (database, Stripe outage, Stripe key missing): nack so Pub/Sub retries, then dead-letters.
    Retry(String),
}

pub struct Intake {
    keys: Keys,
    tenants: Vec<String>,
}

impl Intake {
    pub fn new(keys: Keys, tenants: Vec<String>) -> Self {
        Self { keys, tenants }
    }

    /// The checks that need no database: everything except Stripe and the write.
    pub fn admit(&self, data: &[u8]) -> Result<Screened, Verdict> {
        let screened = screen(data, &self.keys).map_err(|r| Verdict::Drop(r.to_string()))?;
        if !self
            .tenants
            .iter()
            .any(|t| t == &screened.conversion.tenant_id)
        {
            return Err(Verdict::Drop(format!(
                "tenant {} is not served by this intake",
                screened.conversion.tenant_id
            )));
        }
        Ok(screened)
    }

    pub fn handle(&self, data: &[u8]) -> Verdict {
        let screened = match self.admit(data) {
            Ok(s) => s,
            Err(v) => return v,
        };
        let id = screened.conversion.conversion_id;
        let context = session(&screened.conversion);
        let pool = match crate::database::db() {
            Ok(db) => db.pool(),
            Err(e) => return Verdict::Retry(format!("{id}: database unavailable: {e}")),
        };
        match pool
            .with_session_transaction(&context, |executor| accounts::record(executor, &screened))
        {
            Ok(Recorded::Created(account)) => {
                Verdict::Done(format!("{id}: created customer {account}"))
            }
            Ok(Recorded::Linked(account)) => {
                Verdict::Done(format!("{id}: linked to customer {account}"))
            }
            Ok(Recorded::Duplicate(account)) => {
                Verdict::Done(format!("{id}: already recorded (customer {account})"))
            }
            Err(e) => Verdict::Retry(format!("{id}: write failed: {e}")),
        }
    }
}

/// The RLS session the intake writes under: the conversion's tenant and organisation, the intake's own subject.
pub fn session(conversion: &Conversion) -> SessionContext {
    SessionContext {
        tenant_id: conversion.tenant_id.clone(),
        subject_id: INTAKE_SUBJECT,
        organization_id: conversion.organization_id,
        session_id: format!("crm-intake:{}", conversion.conversion_id),
        roles: vec!["crm.intake".to_string()],
        permissions: vec!["crm:accounts:create".to_string()],
        user_type: Some("service".to_string()),
        org_type: None,
    }
}

/// Start the consumer when CRM_CONVERSIONS_SUBSCRIPTION is set; refuse to start half-configured.
pub fn start_if_enabled() {
    let sub = std::env::var("CRM_CONVERSIONS_SUBSCRIPTION").unwrap_or_default();
    if sub.trim().is_empty() {
        println!("crm-intake: CRM_CONVERSIONS_SUBSCRIPTION not set; intake disabled");
        return;
    }
    if !sub.starts_with("projects/") || !sub.contains("/subscriptions/") {
        eprintln!("crm-intake: CRM_CONVERSIONS_SUBSCRIPTION must be projects/<p>/subscriptions/<s>; intake disabled");
        return;
    }
    let keys = match Keys::from_env() {
        Ok(k) if !k.is_empty() => k,
        Ok(_) => {
            eprintln!("crm-intake: CRM_CONVERSION_VERIFY_KEYS is empty; intake disabled");
            return;
        }
        Err(e) => {
            eprintln!("crm-intake: CRM_CONVERSION_VERIFY_KEYS: {e}; intake disabled");
            return;
        }
    };
    let tenants: Vec<String> = std::env::var("CRM_INTAKE_TENANTS")
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .map(str::to_string)
        .collect();
    if tenants.is_empty() {
        eprintln!("crm-intake: CRM_INTAKE_TENANTS is empty; intake disabled");
        return;
    }
    let intake = Intake::new(keys, tenants);
    let _ = std::thread::Builder::new().name("crm-intake".into()).spawn(move || {
        let mut pubsub = match PubSub::new() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("crm-intake: {e}; intake disabled");
                return;
            }
        };
        println!("crm-intake: consuming {sub}");
        loop {
            let messages = match pubsub.pull(&sub, 10) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("crm-intake: pull failed: {e}");
                    std::thread::sleep(Duration::from_secs(5));
                    continue;
                }
            };
            let (mut ack, mut nack) = (Vec::new(), Vec::new());
            for m in messages {
                match intake.handle(&m.data) {
                    Verdict::Done(msg) => {
                        println!("crm-intake: {msg}");
                        ack.push(m.ack_id);
                    }
                    Verdict::Drop(msg) => {
                        eprintln!("crm-intake: dropped: {msg}");
                        ack.push(m.ack_id);
                    }
                    Verdict::Retry(msg) => {
                        eprintln!("crm-intake: retry (attempt {}): {msg}", m.delivery_attempt);
                        nack.push(m.ack_id);
                    }
                }
            }
            if let Err(e) = pubsub.ack(&sub, &ack) {
                eprintln!("crm-intake: ack failed (messages will redeliver; recording is idempotent): {e}");
            }
            if let Err(e) = pubsub.nack(&sub, &nack) {
                eprintln!("crm-intake: nack failed: {e}");
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use rerp_crm_conversion::{manual_conversion_id, Address, Customer, Reason, Sealer, VERSION};
    use serde_json::json;

    fn sealer() -> Sealer {
        Sealer::from_seed_b64("k1", "BwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwc=").unwrap()
    }

    fn conversion(tenant: &str) -> Conversion {
        let lead = Uuid::from_u128(42);
        Conversion {
            version: VERSION,
            conversion_id: manual_conversion_id(lead),
            tenant_id: tenant.into(),
            organization_id: Uuid::from_u128(9),
            reason: Reason::Order,
            reference: "PO-1".into(),
            funnel_lead_id: Some(lead),
            stripe: None,
            customer: Customer {
                name: "Acme".into(),
                contact_name: None,
                email: "a@acme.example".into(),
                address: Address {
                    line1: "1 High St".into(),
                    line2: None,
                    city: "Sofia".into(),
                    postal_code: None,
                    region: None,
                    country: "BG".into(),
                },
                phone: None,
                tax_id: None,
            },
            converted_by: Some("s1".into()),
            converted_at: "2026-10-23T00:00:00Z".parse().unwrap(),
        }
    }

    fn intake() -> Intake {
        let keys = Keys::parse(&format!("k1:{}", sealer().public_b64())).unwrap();
        Intake::new(keys, vec!["pricewhisperer".into()])
    }

    #[test]
    fn admits_a_signed_conversion_for_a_served_tenant() {
        let env = sealer().seal(&conversion("pricewhisperer")).unwrap();
        assert!(intake().admit(&serde_json::to_vec(&env).unwrap()).is_ok());
    }

    #[test]
    fn drops_foreign_tenants_and_garbage() {
        let env = sealer().seal(&conversion("hauliage")).unwrap();
        assert!(matches!(
            intake().admit(&serde_json::to_vec(&env).unwrap()),
            Err(Verdict::Drop(_))
        ));
        assert!(matches!(intake().admit(b"not json"), Err(Verdict::Drop(_))));
    }

    #[test]
    fn session_is_the_tenant_with_the_intake_subject() {
        let s = session(&conversion("pricewhisperer"));
        assert_eq!(s.tenant_id, "pricewhisperer");
        assert_eq!(s.subject_id, INTAKE_SUBJECT);
        assert_eq!(s.organization_id, Uuid::from_u128(9));
    }
}
