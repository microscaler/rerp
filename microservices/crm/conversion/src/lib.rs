//! The conversion contract (ADR 003).
//!
//! A lead in the marketing funnel crosses into RERP only when it converts: a paid Stripe checkout, or staff marking an
//! accepted quote, order or signed contract *won*. The crossing is one message on Pub/Sub `crm-conversions`:
//!
//! ```json
//! { "payload": "<Conversion as a JSON string>", "signature": "<base64 Ed25519>", "key_id": "k1" }
//! ```
//!
//! The signature covers the payload string's exact bytes, so nothing is re-serialised before checking. Every conversion
//! is signed, and a key's id says what it may sign: `billing*` keys (PriceWhisperer billing, which has re-retrieved
//! the checkout from Stripe) sign paid Stripe checkouts only; every other key (the CRM pipeline) signs manual
//! conversions only. Everything here is pure: screening needs nothing but the bytes and the public keys.

pub mod provision;
pub mod pubsub;

use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use chrono::{DateTime, Utc};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

/// Largest message the intake will look at.
pub const MAX_MESSAGE_BYTES: usize = 16 * 1024;
/// Largest payload inside it.
pub const MAX_PAYLOAD_BYTES: usize = 12 * 1024;
pub const VERSION: u32 = 1;

/// Namespace for deterministic conversion ids: one conversion per lead, one per Stripe checkout.
const ID_NAMESPACE: Uuid = Uuid::from_u128(0x6a1f_0c2e_9b4d_4e57_8f3a_1c2d_3e4f_5a6b);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Reason {
    /// A paid Stripe checkout (automatic, published by billing).
    StripeCheckout,
    /// An accepted quote or order, marked won by staff.
    Order,
    /// A signed contract, marked won by staff.
    Contract,
}

impl Reason {
    pub fn is_manual(self) -> bool {
        !matches!(self, Reason::StripeCheckout)
    }
    pub fn as_str(self) -> &'static str {
        match self {
            Reason::StripeCheckout => "stripe_checkout",
            Reason::Order => "order",
            Reason::Contract => "contract",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StripeRef {
    pub customer_id: String,
    pub checkout_session_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Address {
    pub line1: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    pub city: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// ISO 3166-1 alpha-2, upper case.
    pub country: String,
}

/// What crosses: enough to invoice and chase, nothing from the marketing history.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Customer {
    /// Legal or company name (a person's name for an individual).
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    pub email: String,
    /// As the customer gave it (Stripe collects it at checkout).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    pub address: Address,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Conversion {
    pub version: u32,
    pub conversion_id: Uuid,
    /// The RERP tenant the customer converts into.
    pub tenant_id: String,
    /// The tenant's organisation that will own the customer.
    pub organization_id: Uuid,
    pub reason: Reason,
    /// The order, contract or checkout reference.
    pub reference: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub funnel_lead_id: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stripe: Option<StripeRef>,
    pub customer: Customer,
    /// The staff member (Sesame subject) for a manual conversion.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub converted_by: Option<String>,
    pub converted_at: DateTime<Utc>,
}

/// The id a manual conversion of a funnel lead always gets: re-publishing the same lead is the same conversion.
pub fn manual_conversion_id(funnel_lead_id: Uuid) -> Uuid {
    Uuid::new_v5(&ID_NAMESPACE, format!("lead:{funnel_lead_id}").as_bytes())
}

/// The organisation that owns a tenant's customers when no Sesame organisation is in play (a tenant whose staff
/// belong to no organisation, or a Stripe checkout): one fixed id per tenant, the same wherever it is computed.
pub fn tenant_organization_id(tenant: &str) -> Uuid {
    Uuid::new_v5(&ID_NAMESPACE, format!("tenant:{tenant}").as_bytes())
}

/// The id a Stripe conversion always gets: one per checkout session.
pub fn stripe_conversion_id(checkout_session_id: &str) -> Uuid {
    Uuid::new_v5(
        &ID_NAMESPACE,
        format!("stripe:{checkout_session_id}").as_bytes(),
    )
}

/// The wire envelope.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Envelope {
    pub payload: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Reject {
    TooLarge,
    Malformed(String),
    Invalid(String),
    Unsigned,
    UnknownKey(String),
    /// The key may not sign this kind of conversion.
    WrongKey(String),
    BadSignature,
}

impl std::fmt::Display for Reject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reject::TooLarge => f.write_str("message too large"),
            Reject::Malformed(m) => write!(f, "malformed: {m}"),
            Reject::Invalid(m) => write!(f, "invalid: {m}"),
            Reject::Unsigned => f.write_str("conversion is not signed"),
            Reject::UnknownKey(k) => write!(f, "unknown signing key {k}"),
            Reject::WrongKey(k) => write!(f, "key {k} may not sign this kind of conversion"),
            Reject::BadSignature => f.write_str("signature does not verify"),
        }
    }
}

fn check_len(field: &str, value: &str, min: usize, max: usize) -> Result<(), Reject> {
    let n = value.chars().count();
    if n < min || n > max {
        return Err(Reject::Invalid(format!(
            "{field} must be {min}..={max} characters"
        )));
    }
    if value.chars().any(|c| c.is_control()) {
        return Err(Reject::Invalid(format!(
            "{field} contains control characters"
        )));
    }
    Ok(())
}

fn check_opt(field: &str, value: &Option<String>, max: usize) -> Result<(), Reject> {
    match value {
        Some(v) => check_len(field, v, 1, max),
        None => Ok(()),
    }
}

fn check_email(value: &str) -> Result<(), Reject> {
    check_len("customer.email", value, 3, 254)?;
    let mut parts = value.split('@');
    let (local, domain) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""));
    if parts.next().is_some()
        || local.is_empty()
        || !domain.contains('.')
        || domain.starts_with('.')
        || domain.ends_with('.')
        || value.chars().any(char::is_whitespace)
    {
        return Err(Reject::Invalid("customer.email is not an address".into()));
    }
    Ok(())
}

impl Conversion {
    /// Field-level checks. Format, not truth: a manual conversion's details are what staff typed.
    pub fn validate(&self) -> Result<(), Reject> {
        if self.version != VERSION {
            return Err(Reject::Invalid(format!(
                "unsupported version {}",
                self.version
            )));
        }
        check_len("tenant_id", &self.tenant_id, 1, 64)?;
        if !self
            .tenant_id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
        {
            return Err(Reject::Invalid(
                "tenant_id has unexpected characters".into(),
            ));
        }
        check_len("reference", &self.reference, 1, 200)?;
        let c = &self.customer;
        check_len("customer.name", &c.name, 1, 200)?;
        check_opt("customer.contact_name", &c.contact_name, 200)?;
        check_email(&c.email)?;
        check_opt("customer.phone", &c.phone, 40)?;
        check_opt("customer.tax_id", &c.tax_id, 50)?;
        let a = &c.address;
        check_len("address.line1", &a.line1, 1, 200)?;
        check_opt("address.line2", &a.line2, 200)?;
        check_len("address.city", &a.city, 1, 100)?;
        check_opt("address.postal_code", &a.postal_code, 20)?;
        check_opt("address.region", &a.region, 100)?;
        if a.country.len() != 2 || !a.country.chars().all(|ch| ch.is_ascii_uppercase()) {
            return Err(Reject::Invalid(
                "address.country must be ISO 3166-1 alpha-2".into(),
            ));
        }
        check_opt("converted_by", &self.converted_by, 200)?;
        if self.reason.is_manual() {
            let Some(lead) = self.funnel_lead_id else {
                return Err(Reject::Invalid(
                    "a manual conversion names its funnel lead".into(),
                ));
            };
            if self.stripe.is_some() {
                return Err(Reject::Invalid(
                    "a manual conversion carries no Stripe reference".into(),
                ));
            }
            if self.converted_by.is_none() {
                return Err(Reject::Invalid(
                    "a manual conversion names who converted it".into(),
                ));
            }
            if self.conversion_id != manual_conversion_id(lead) {
                return Err(Reject::Invalid(
                    "conversion_id does not match the lead".into(),
                ));
            }
        } else {
            let Some(s) = &self.stripe else {
                return Err(Reject::Invalid(
                    "a Stripe conversion carries its Stripe ids".into(),
                ));
            };
            check_len("stripe.customer_id", &s.customer_id, 5, 255)?;
            check_len("stripe.checkout_session_id", &s.checkout_session_id, 5, 255)?;
            if !s.customer_id.starts_with("cus_") || !s.checkout_session_id.starts_with("cs_") {
                return Err(Reject::Invalid("Stripe ids have the wrong prefix".into()));
            }
            if self.conversion_id != stripe_conversion_id(&s.checkout_session_id) {
                return Err(Reject::Invalid(
                    "conversion_id does not match the checkout".into(),
                ));
            }
        }
        Ok(())
    }
}

/// The pipeline's signing key.
pub struct Sealer {
    key_id: String,
    key: SigningKey,
}

impl Sealer {
    /// From a base64 32-byte Ed25519 seed.
    pub fn from_seed_b64(key_id: &str, seed_b64: &str) -> Result<Self, String> {
        let bytes = B64
            .decode(seed_b64.trim())
            .map_err(|_| "signing key is not base64".to_string())?;
        let seed: [u8; 32] = bytes
            .try_into()
            .map_err(|_| "signing key must be 32 bytes".to_string())?;
        check_key_id(key_id)?;
        Ok(Self {
            key_id: key_id.to_string(),
            key: SigningKey::from_bytes(&seed),
        })
    }

    /// `CRM_CONVERSION_KEY_ID` and `CRM_CONVERSION_SIGNING_KEY`.
    pub fn from_env() -> Result<Self, String> {
        let id = std::env::var("CRM_CONVERSION_KEY_ID")
            .map_err(|_| "CRM_CONVERSION_KEY_ID is not set")?;
        let seed = std::env::var("CRM_CONVERSION_SIGNING_KEY")
            .map_err(|_| "CRM_CONVERSION_SIGNING_KEY is not set")?;
        Self::from_seed_b64(&id, &seed)
    }

    pub fn public_b64(&self) -> String {
        B64.encode(self.key.verifying_key().to_bytes())
    }

    /// Validate, serialise once and sign those exact bytes.
    pub fn seal(&self, conversion: &Conversion) -> Result<Envelope, Reject> {
        conversion.validate()?;
        let payload =
            serde_json::to_string(conversion).map_err(|e| Reject::Malformed(e.to_string()))?;
        if payload.len() > MAX_PAYLOAD_BYTES {
            return Err(Reject::TooLarge);
        }
        let signature = self.key.sign(payload.as_bytes());
        Ok(Envelope {
            payload,
            signature: Some(B64.encode(signature.to_bytes())),
            key_id: Some(self.key_id.clone()),
        })
    }
}

fn check_key_id(key_id: &str) -> Result<(), String> {
    if key_id.is_empty()
        || key_id.len() > 32
        || !key_id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-')
    {
        return Err("key id must be 1..=32 of [A-Za-z0-9-]".into());
    }
    Ok(())
}

/// Public keys the intake trusts, by key id (more than one while a key rolls).
#[derive(Default)]
pub struct Keys(BTreeMap<String, VerifyingKey>);

impl Keys {
    /// `k1:<base64 public key>,k2:<base64 public key>`
    pub fn parse(spec: &str) -> Result<Self, String> {
        let mut keys = BTreeMap::new();
        for item in spec.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            let (id, b64) = item
                .split_once(':')
                .ok_or("key entries are <id>:<base64>")?;
            check_key_id(id)?;
            let bytes = B64
                .decode(b64.trim())
                .map_err(|_| format!("key {id} is not base64"))?;
            let raw: [u8; 32] = bytes
                .try_into()
                .map_err(|_| format!("key {id} must be 32 bytes"))?;
            let key = VerifyingKey::from_bytes(&raw)
                .map_err(|_| format!("key {id} is not a valid Ed25519 key"))?;
            keys.insert(id.to_string(), key);
        }
        Ok(Self(keys))
    }

    /// `CRM_CONVERSION_VERIFY_KEYS` and any `CRM_CONVERSION_VERIFY_KEYS_*` (one per signer's secret).
    pub fn from_env() -> Result<Self, String> {
        let mut specs: Vec<(String, String)> = std::env::vars()
            .filter(|(k, _)| k.starts_with("CRM_CONVERSION_VERIFY_KEYS"))
            .collect();
        specs.sort();
        Self::parse(
            &specs
                .into_iter()
                .map(|(_, v)| v)
                .collect::<Vec<_>>()
                .join(","),
        )
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Whether the key `key_id` may sign a conversion for `reason`.
pub fn key_may_sign(key_id: &str, reason: Reason) -> bool {
    key_id.starts_with("billing") != reason.is_manual()
}

/// A screened conversion: valid, and signed by a trusted key allowed to sign it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Screened {
    pub conversion: Conversion,
    pub signed_by: Option<String>,
}

/// Every check that needs nothing but the bytes and the public keys.
pub fn screen(message: &[u8], keys: &Keys) -> Result<Screened, Reject> {
    if message.len() > MAX_MESSAGE_BYTES {
        return Err(Reject::TooLarge);
    }
    let envelope: Envelope =
        serde_json::from_slice(message).map_err(|e| Reject::Malformed(e.to_string()))?;
    if envelope.payload.len() > MAX_PAYLOAD_BYTES {
        return Err(Reject::TooLarge);
    }
    let conversion: Conversion =
        serde_json::from_str(&envelope.payload).map_err(|e| Reject::Malformed(e.to_string()))?;
    conversion.validate()?;
    let signed_by = match (&envelope.signature, &envelope.key_id) {
        (Some(sig), Some(id)) => {
            let key = keys
                .0
                .get(id)
                .ok_or_else(|| Reject::UnknownKey(id.clone()))?;
            let raw = B64.decode(sig).map_err(|_| Reject::BadSignature)?;
            let raw: [u8; 64] = raw.try_into().map_err(|_| Reject::BadSignature)?;
            key.verify(envelope.payload.as_bytes(), &Signature::from_bytes(&raw))
                .map_err(|_| Reject::BadSignature)?;
            Some(id.clone())
        }
        (None, None) => None,
        _ => return Err(Reject::Malformed("signature and key_id go together".into())),
    };
    let Some(key_id) = signed_by.clone() else {
        return Err(Reject::Unsigned);
    };
    if !key_may_sign(&key_id, conversion.reason) {
        return Err(Reject::WrongKey(key_id));
    }
    Ok(Screened {
        conversion,
        signed_by,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed() -> String {
        B64.encode([7u8; 32])
    }

    fn manual() -> Conversion {
        let lead = Uuid::from_u128(42);
        Conversion {
            version: VERSION,
            conversion_id: manual_conversion_id(lead),
            tenant_id: "pricewhisperer".into(),
            organization_id: Uuid::from_u128(9),
            reason: Reason::Contract,
            reference: "MSA-2026-014".into(),
            funnel_lead_id: Some(lead),
            stripe: None,
            customer: Customer {
                name: "Acme Trading Ltd".into(),
                contact_name: Some("Ada Lovelace".into()),
                email: "accounts@acme.example".into(),
                address: Address {
                    line1: "1 High Street".into(),
                    line2: None,
                    city: "Sofia".into(),
                    postal_code: Some("1000".into()),
                    region: None,
                    country: "BG".into(),
                },
                phone: None,
                tax_id: Some("BG123456789".into()),
            },
            converted_by: Some("a1000001-0001-4000-8000-000000000004".into()),
            converted_at: "2026-10-23T00:00:00Z".parse().unwrap(),
        }
    }

    fn keys(s: &Sealer) -> Keys {
        Keys::parse(&format!("k1:{}", s.public_b64())).unwrap()
    }

    #[test]
    fn a_signed_manual_conversion_passes() {
        let s = Sealer::from_seed_b64("k1", &seed()).unwrap();
        let env = s.seal(&manual()).unwrap();
        let bytes = serde_json::to_vec(&env).unwrap();
        let got = screen(&bytes, &keys(&s)).unwrap();
        assert_eq!(got.conversion, manual());
        assert_eq!(got.signed_by.as_deref(), Some("k1"));
    }

    #[test]
    fn tampering_unsigned_and_unknown_keys_are_rejected() {
        let s = Sealer::from_seed_b64("k1", &seed()).unwrap();
        let mut env = s.seal(&manual()).unwrap();
        env.payload = env.payload.replace("Acme", "Evil");
        assert_eq!(
            screen(&serde_json::to_vec(&env).unwrap(), &keys(&s)),
            Err(Reject::BadSignature)
        );

        let mut env = s.seal(&manual()).unwrap();
        env.signature = None;
        env.key_id = None;
        assert_eq!(
            screen(&serde_json::to_vec(&env).unwrap(), &keys(&s)),
            Err(Reject::Unsigned)
        );

        let other = Sealer::from_seed_b64("k2", &B64.encode([8u8; 32])).unwrap();
        let env = other.seal(&manual()).unwrap();
        assert_eq!(
            screen(&serde_json::to_vec(&env).unwrap(), &keys(&s)),
            Err(Reject::UnknownKey("k2".into()))
        );
    }

    #[test]
    fn unknown_fields_oversize_and_bad_details_are_rejected() {
        let s = Sealer::from_seed_b64("k1", &seed()).unwrap();
        let env = s.seal(&manual()).unwrap();
        let mut v: serde_json::Value = serde_json::to_value(&env).unwrap();
        v["extra"] = 1.into();
        assert!(matches!(
            screen(v.to_string().as_bytes(), &keys(&s)),
            Err(Reject::Malformed(_))
        ));
        assert_eq!(
            screen(&vec![b' '; MAX_MESSAGE_BYTES + 1], &keys(&s)),
            Err(Reject::TooLarge)
        );

        for bad in [
            {
                let mut c = manual();
                c.customer.email = "not an email".into();
                c
            },
            {
                let mut c = manual();
                c.customer.address.country = "bg".into();
                c
            },
            {
                let mut c = manual();
                c.conversion_id = Uuid::from_u128(1);
                c
            },
            {
                let mut c = manual();
                c.converted_by = None;
                c
            },
        ] {
            assert!(matches!(s.seal(&bad), Err(Reject::Invalid(_))));
        }
    }

    fn stripe() -> Conversion {
        let mut c = manual();
        c.reason = Reason::StripeCheckout;
        c.funnel_lead_id = None;
        c.converted_by = None;
        c.customer.phone = Some("+359 88 123 4567".into());
        c.stripe = Some(StripeRef {
            customer_id: "cus_123".into(),
            checkout_session_id: "cs_test_1".into(),
        });
        c.conversion_id = stripe_conversion_id("cs_test_1");
        c
    }

    #[test]
    fn keys_sign_only_their_kind_of_conversion() {
        let pipeline = Sealer::from_seed_b64("k1", &seed()).unwrap();
        let billing = Sealer::from_seed_b64("billing1", &B64.encode([9u8; 32])).unwrap();
        let keys = Keys::parse(&format!(
            "k1:{},billing1:{}",
            pipeline.public_b64(),
            billing.public_b64()
        ))
        .unwrap();
        let ok = |s: &Sealer, c: &Conversion| {
            screen(&serde_json::to_vec(&s.seal(c).unwrap()).unwrap(), &keys)
        };
        assert_eq!(
            ok(&billing, &stripe()).unwrap().signed_by.as_deref(),
            Some("billing1")
        );
        assert!(ok(&pipeline, &manual()).is_ok());
        assert_eq!(ok(&pipeline, &stripe()), Err(Reject::WrongKey("k1".into())));
        assert_eq!(
            ok(&billing, &manual()),
            Err(Reject::WrongKey("billing1".into()))
        );
        let unsigned = Envelope {
            payload: serde_json::to_string(&stripe()).unwrap(),
            signature: None,
            key_id: None,
        };
        assert_eq!(
            screen(&serde_json::to_vec(&unsigned).unwrap(), &keys),
            Err(Reject::Unsigned)
        );
    }

    /// The payload PriceWhisperer billing writes (its own serialiser; no shared crate) must screen here unchanged.
    /// Keep in step with billing's conversions.rs test of the same name.
    #[test]
    fn billings_wire_format_screens() {
        let id = stripe_conversion_id("cs_test_abc").to_string();
        let payload = BILLING_FIXTURE.replace("CONV", &id);
        let key = SigningKey::from_bytes(&[9u8; 32]);
        let env = Envelope {
            signature: Some(B64.encode(key.sign(payload.as_bytes()).to_bytes())),
            payload,
            key_id: Some("billing1".into()),
        };
        let keys = Keys::parse(&format!(
            "billing1:{}",
            B64.encode(key.verifying_key().to_bytes())
        ))
        .unwrap();
        let got = screen(&serde_json::to_vec(&env).unwrap(), &keys).unwrap();
        assert_eq!(got.conversion.reason, Reason::StripeCheckout);
        assert_eq!(
            got.conversion.customer.phone.as_deref(),
            Some("+359881234567")
        );
        assert_eq!(
            got.conversion.customer.tax_id.as_deref(),
            Some("BG123456789")
        );
    }

    const BILLING_FIXTURE: &str = r#"{"version":1,"conversion_id":"CONV","tenant_id":"pricewhisperer","organization_id":"00000000-0000-4000-8000-000000000001","reason":"stripe_checkout","reference":"cs_test_abc","stripe":{"customer_id":"cus_abc","checkout_session_id":"cs_test_abc"},"customer":{"name":"Ada Trading Ltd","contact_name":"Ada Lovelace","email":"ada@example.com","phone":"+359881234567","address":{"line1":"1 High Street","city":"Sofia","postal_code":"1000","country":"BG"},"tax_id":"BG123456789"},"converted_at":"2026-10-23T08:00:00Z"}"#;

    /// The ids PriceWhisperer billing computes on its own (its conversions.rs tests assert the same literals).
    #[test]
    fn ids_billing_also_computes() {
        assert_eq!(
            stripe_conversion_id("cs_test_abc").to_string(),
            "b14363bd-ecd1-5883-a7da-56000f3bc073"
        );
        assert_eq!(
            tenant_organization_id("pricewhisperer").to_string(),
            "143b71df-2d22-53d3-8d50-bfede26eb119"
        );
    }

    #[test]
    fn ids_are_deterministic() {
        let lead = Uuid::from_u128(42);
        assert_eq!(manual_conversion_id(lead), manual_conversion_id(lead));
        assert_ne!(manual_conversion_id(lead), stripe_conversion_id("cs_x"));
    }
}
