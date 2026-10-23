//! The customer record in RERP's database (ADR 003): created from conversions, read by staff and accounting.
//!
//! Plain SQL over Lifeguard's executor. Parameters are bound as text (`sea_query::Values`, which cross the pool's
//! channel) and cast in SQL, and values come back as text, so nothing depends on driver type features. Every call runs
//! inside a Sesame RLS session transaction.

use lifeguard::{LifeError, LifeExecutor};
use rerp_crm_conversion::Screened;
use sea_query::{Value as Param, Values};
use serde_json::{json, Value};
use uuid::Uuid;

fn text(value: &str) -> Param {
    Param::from(value.to_string())
}

fn opt(value: &Option<String>) -> Param {
    Param::from(value.clone())
}

/// What recording a conversion did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Recorded {
    /// A new customer.
    Created(Uuid),
    /// An existing customer (same funnel lead or Stripe customer) now carries this conversion too.
    Linked(Uuid),
    /// This conversion was recorded before: nothing changed.
    Duplicate(Uuid),
}

fn uuid_of(text: &str) -> Result<Uuid, LifeError> {
    Uuid::parse_str(text).map_err(|e| LifeError::Other(format!("bad uuid from database: {e}")))
}

/// Record a screened, verified conversion exactly once.
pub fn record<E: LifeExecutor>(executor: &E, screened: &Screened) -> Result<Recorded, LifeError> {
    let c = &screened.conversion;
    let conversion_id = c.conversion_id.to_string();
    let tenant = c.tenant_id.as_str();
    let lead = c.funnel_lead_id.map(|l| l.to_string());
    let stripe_customer = c.stripe.as_ref().map(|s| s.customer_id.clone());

    let seen = executor.query_all_values(
        "SELECT account_id::text FROM crm_conversions WHERE conversion_id = $1::text::uuid",
        &Values(vec![text(&conversion_id)]),
    )?;
    if let Some(row) = seen.first() {
        return Ok(Recorded::Duplicate(uuid_of(&row.get::<_, String>(0))?));
    }

    let existing = executor.query_all_values(
        "SELECT id::text FROM crm_accounts WHERE tenant_id = $1::text AND \
         ((funnel_lead_id IS NOT NULL AND funnel_lead_id = $2::text::uuid) OR \
          (stripe_customer_id IS NOT NULL AND stripe_customer_id = $3::text)) \
         ORDER BY created_at LIMIT 1",
        &Values(vec![text(tenant), opt(&lead), opt(&stripe_customer)]),
    )?;
    let (account, outcome) = match existing.first() {
        Some(row) => {
            let id = uuid_of(&row.get::<_, String>(0))?;
            (id, Recorded::Linked(id))
        }
        None => {
            let id = Uuid::new_v4();
            let cu = &c.customer;
            let a = &cu.address;
            executor.execute_values(
                "INSERT INTO crm_accounts (id, tenant_id, organization_id, name, contact_name, email, street, street2, \
                 city, zip, region, country_code, tax_id, funnel_lead_id, stripe_customer_id, phone) \
                 VALUES ($1::text::uuid, $2::text, $3::text::uuid, $4::text, $5::text, $6::text, $7::text, $8::text, \
                 $9::text, $10::text, $11::text, $12::text, $13::text, $14::text::uuid, $15::text, $16::text)",
                &Values(vec![
                    text(&id.to_string()),
                    text(tenant),
                    text(&c.organization_id.to_string()),
                    text(&cu.name),
                    opt(&cu.contact_name),
                    text(&cu.email),
                    text(&a.line1),
                    opt(&a.line2),
                    text(&a.city),
                    opt(&a.postal_code),
                    opt(&a.region),
                    text(&a.country),
                    opt(&cu.tax_id),
                    opt(&lead),
                    opt(&stripe_customer),
                    opt(&cu.phone),
                ]),
            )?;
            (id, Recorded::Created(id))
        }
    };

    let payload = serde_json::to_string(c).map_err(|e| LifeError::Other(e.to_string()))?;
    executor.execute_values(
        "INSERT INTO crm_conversions (conversion_id, tenant_id, account_id, reason, reference, funnel_lead_id, \
         stripe_customer_id, converted_by, converted_at, signed_by, payload) \
         VALUES ($1::text::uuid, $2::text, $3::text::uuid, $4::text, $5::text, $6::text::uuid, $7::text, $8::text, \
         $9::text::timestamptz, $10::text, $11::text::jsonb)",
        &Values(vec![
            text(&conversion_id),
            text(tenant),
            text(&account.to_string()),
            text(c.reason.as_str()),
            text(&c.reference),
            opt(&lead),
            opt(&stripe_customer),
            opt(&c.converted_by),
            text(&c.converted_at.to_rfc3339()),
            opt(&screened.signed_by),
            text(&payload),
        ]),
    )?;
    Ok(outcome)
}

const ACCOUNT_COLUMNS: &str = "a.id::text, a.name, a.contact_name, a.email, a.street, a.street2, a.city, a.zip, \
     a.region, a.country_code::text, a.tax_id, a.active, \
     to_char(a.created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD\"T\"HH24:MI:SS\"Z\"'), \
     to_char(a.updated_at AT TIME ZONE 'UTC', 'YYYY-MM-DD\"T\"HH24:MI:SS\"Z\"'), \
     o.conversion_id::text, o.reason, o.reference, o.funnel_lead_id::text, o.stripe_customer_id, o.converted_by, \
     to_char(o.converted_at AT TIME ZONE 'UTC', 'YYYY-MM-DD\"T\"HH24:MI:SS\"Z\"'), a.phone";

/// The first conversion of each account is its origin.
const ORIGIN_JOIN: &str =
    "LEFT JOIN LATERAL (SELECT * FROM crm_conversions c WHERE c.tenant_id = a.tenant_id \
     AND c.account_id = a.id ORDER BY c.converted_at, c.conversion_id LIMIT 1) o ON true";

fn account_json(row: &lifeguard::may_postgres::Row) -> Value {
    let s = |i: usize| row.get::<_, Option<String>>(i);
    let mut v = json!({
        "id": s(0),
        "name": s(1),
        "email": s(3),
        "street": s(4),
        "city": s(6),
        "country_code": s(9),
        "active": row.get::<_, bool>(11),
        "create_date": s(12),
        "write_date": s(13),
    });
    for (key, i) in [
        ("contact_name", 2),
        ("street2", 5),
        ("zip", 7),
        ("region", 8),
        ("tax_id", 10),
        ("phone", 21),
    ] {
        if let Some(value) = s(i) {
            v[key] = json!(value);
        }
    }
    if let Some(conversion_id) = s(14) {
        let mut origin = json!({
            "conversion_id": conversion_id,
            "reason": s(15),
            "reference": s(16),
            "converted_at": s(20),
        });
        for (key, i) in [
            ("funnel_lead_id", 17),
            ("stripe_customer_id", 18),
            ("converted_by", 19),
        ] {
            if let Some(value) = s(i) {
                origin[key] = json!(value);
            }
        }
        v["origin"] = origin;
    }
    v
}

/// Paginated accounts, by name, optionally filtered by a name or email search and active flag.
pub fn list<E: LifeExecutor>(
    executor: &E,
    search: Option<&str>,
    active: Option<bool>,
    page: u32,
    limit: u32,
) -> Result<Value, LifeError> {
    let pattern = search.map(|s| {
        format!(
            "%{}%",
            s.replace('\\', "\\\\")
                .replace('%', "\\%")
                .replace('_', "\\_")
        )
    });
    let filter = "($1::text IS NULL OR a.name ILIKE $1::text OR a.email ILIKE $1::text) \
                  AND ($2::text IS NULL OR a.active = ($2::text = 'true'))";
    let active_text = active.map(|a| a.to_string());
    let total: i64 = executor
        .query_one_values(
            &format!("SELECT count(*) FROM crm_accounts a WHERE {filter}"),
            &Values(vec![opt(&pattern), opt(&active_text)]),
        )?
        .get(0);
    let offset = (i64::from(page) - 1) * i64::from(limit);
    let rows = executor.query_all_values(
        &format!(
            "SELECT {ACCOUNT_COLUMNS} FROM crm_accounts a {ORIGIN_JOIN} WHERE {filter} \
             ORDER BY lower(a.name), a.id LIMIT $3::text::bigint OFFSET $4::text::bigint"
        ),
        &Values(vec![
            opt(&pattern),
            opt(&active_text),
            text(&limit.to_string()),
            text(&offset.to_string()),
        ]),
    )?;
    let items: Vec<Value> = rows.iter().map(account_json).collect();
    let shown = offset + items.len() as i64;
    Ok(
        json!({ "items": items, "total": total, "page": page, "limit": limit, "has_more": shown < total }),
    )
}

pub fn get<E: LifeExecutor>(executor: &E, id: Uuid) -> Result<Option<Value>, LifeError> {
    let rows = executor.query_all_values(
        &format!("SELECT {ACCOUNT_COLUMNS} FROM crm_accounts a {ORIGIN_JOIN} WHERE a.id = $1::text::uuid"),
        &Values(vec![text(&id.to_string())]),
    )?;
    Ok(rows.first().map(account_json))
}
