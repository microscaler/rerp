//! Converting a lead (ADR 003): the one way a funnel lead crosses into RERP.
//!
//! Staff mark a lead won with the billing details RERP needs. The pipeline seals a signed conversion
//! (`rerp_crm_conversion`), publishes it to Pub/Sub `crm-conversions`, then marks the lead converted in the funnel.
//! RERP pulls the topic and creates the customer; nothing here holds a RERP credential. The conversion id is
//! deterministic per lead, so a retry (or converting the same lead again) is the same conversion.
//!
//! Env: CRM_CONVERSIONS_TOPIC (projects/<p>/topics/crm-conversions), CRM_CONVERSION_KEY_ID and
//! CRM_CONVERSION_SIGNING_KEY (base64 Ed25519 seed). Pub/Sub credentials: the pod's Workload Identity.

use chrono::Utc;
use rerp_crm_conversion::pubsub::PubSub;
use rerp_crm_conversion::{
    manual_conversion_id, Address, Conversion, Customer, Reason, Reject, Sealer, VERSION,
};
use rerp_crm_pipeline_gen::handlers::types::ConversionCustomer;
use serde_json::Value;
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

/// The staff member converting, from their verified Sesame claims.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Staff {
    pub subject: String,
    pub tenant: String,
    pub organization: Uuid,
}

pub fn staff(claims: Option<&Value>) -> Result<Staff, String> {
    let claims = claims.ok_or("missing verified token claims")?;
    let text = |k: &str| {
        claims
            .get(k)
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|s| !s.is_empty())
    };
    let subject = text("sub").ok_or("the token has no subject")?.to_string();
    let tenant = text("tenant_id")
        .ok_or("the token has no tenant")?
        .to_string();
    let organization = text("org_id")
        .and_then(|o| Uuid::parse_str(o).ok())
        .ok_or("the token has no organisation to own the customer")?;
    Ok(Staff {
        subject,
        tenant,
        organization,
    })
}

pub fn reason(value: &str) -> Option<Reason> {
    match value {
        "order" => Some(Reason::Order),
        "contract" => Some(Reason::Contract),
        _ => None,
    }
}

fn clean(value: &str) -> String {
    value.trim().to_string()
}

fn clean_opt(value: &Option<String>) -> Option<String> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(str::to_string)
}

pub fn build(
    lead: Uuid,
    staff: &Staff,
    reason: Reason,
    reference: &str,
    customer: &ConversionCustomer,
) -> Conversion {
    let a = &customer.address;
    Conversion {
        version: VERSION,
        conversion_id: manual_conversion_id(lead),
        tenant_id: staff.tenant.clone(),
        organization_id: staff.organization,
        reason,
        reference: clean(reference),
        funnel_lead_id: Some(lead),
        stripe: None,
        customer: Customer {
            name: clean(&customer.name),
            contact_name: clean_opt(&customer.contact_name),
            email: clean(&customer.email),
            address: Address {
                line1: clean(&a.line1),
                line2: clean_opt(&a.line2),
                city: clean(&a.city),
                postal_code: clean_opt(&a.postal_code),
                region: clean_opt(&a.region),
                country: clean(&a.country).to_ascii_uppercase(),
            },
            tax_id: clean_opt(&customer.tax_id),
        },
        converted_by: Some(staff.subject.clone()),
        converted_at: Utc::now(),
    }
}

pub enum PublishError {
    /// The details fail the contract's checks: the caller's to fix.
    Invalid(String),
    /// Not configured, or Pub/Sub refused: nothing was published, retry later.
    Unavailable(String),
}

fn sealer() -> Result<&'static Sealer, String> {
    static SEALER: OnceLock<Result<Sealer, String>> = OnceLock::new();
    SEALER
        .get_or_init(Sealer::from_env)
        .as_ref()
        .map_err(Clone::clone)
}

fn topic() -> Result<String, String> {
    let t = std::env::var("CRM_CONVERSIONS_TOPIC").unwrap_or_default();
    if t.starts_with("projects/") && t.contains("/topics/") {
        Ok(t)
    } else {
        Err("CRM_CONVERSIONS_TOPIC is not set to projects/<p>/topics/<t>".into())
    }
}

/// Seal and publish; returns the Pub/Sub message id.
pub fn publish(conversion: &Conversion) -> Result<String, PublishError> {
    let sealer = sealer().map_err(PublishError::Unavailable)?;
    let topic = topic().map_err(PublishError::Unavailable)?;
    let envelope = sealer.seal(conversion).map_err(|e| match e {
        Reject::Invalid(m) => PublishError::Invalid(m),
        other => PublishError::Invalid(other.to_string()),
    })?;
    let bytes =
        serde_json::to_vec(&envelope).map_err(|e| PublishError::Unavailable(e.to_string()))?;
    static CLIENT: OnceLock<Mutex<Option<PubSub>>> = OnceLock::new();
    let mut guard = CLIENT
        .get_or_init(|| Mutex::new(None))
        .lock()
        .unwrap_or_else(|p| p.into_inner());
    if guard.is_none() {
        *guard = Some(PubSub::new().map_err(PublishError::Unavailable)?);
    }
    let id = conversion.conversion_id.to_string();
    guard
        .as_mut()
        .expect("client initialised above")
        .publish(
            &topic,
            &bytes,
            &[
                ("conversion_id", &id),
                ("reason", conversion.reason.as_str()),
                ("tenant_id", &conversion.tenant_id),
            ],
        )
        .map_err(PublishError::Unavailable)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rerp_crm_pipeline_gen::handlers::types::ConversionAddress;
    use serde_json::json;

    fn customer() -> ConversionCustomer {
        ConversionCustomer {
            name: " Acme Trading Ltd ".into(),
            contact_name: Some("".into()),
            email: "accounts@acme.example".into(),
            address: ConversionAddress {
                line1: "1 High Street".into(),
                line2: None,
                city: "Sofia".into(),
                postal_code: Some("1000".into()),
                region: None,
                country: "bg".into(),
            },
            tax_id: None,
        }
    }

    #[test]
    fn staff_needs_subject_tenant_and_organisation() {
        let ok = json!({"sub": "s1", "tenant_id": "pricewhisperer", "org_id": "b2000002-0002-4000-8000-000000000002"});
        assert_eq!(staff(Some(&ok)).unwrap().tenant, "pricewhisperer");
        let no_org = json!({"sub": "s1", "tenant_id": "pricewhisperer"});
        assert!(staff(Some(&no_org)).is_err());
        assert!(staff(None).is_err());
    }

    #[test]
    fn build_cleans_details_and_validates() {
        let s = Staff {
            subject: "s1".into(),
            tenant: "pricewhisperer".into(),
            organization: Uuid::from_u128(1),
        };
        let lead = Uuid::from_u128(7);
        let c = build(lead, &s, Reason::Order, " PO-1 ", &customer());
        assert_eq!(c.customer.name, "Acme Trading Ltd");
        assert_eq!(c.customer.contact_name, None);
        assert_eq!(c.customer.address.country, "BG");
        assert_eq!(c.reference, "PO-1");
        assert_eq!(c.conversion_id, manual_conversion_id(lead));
        assert!(c.validate().is_ok());
    }

    #[test]
    fn only_manual_reasons_are_accepted_here() {
        assert_eq!(reason("order"), Some(Reason::Order));
        assert_eq!(reason("contract"), Some(Reason::Contract));
        assert_eq!(reason("stripe_checkout"), None);
    }
}
