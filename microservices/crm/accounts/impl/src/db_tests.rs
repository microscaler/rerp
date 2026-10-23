//! Database tests for the customer record: run against a real Postgres as a login that is not a superuser (superusers
//! bypass RLS) and may create in schema public, as the platform's owner group does. The test applies the schema itself. Ignored by default:
//! `DB_HOST=... DB_PORT=... DB_USER=... DB_PASS=... DB_NAME=... cargo test -p rerp_crm_accounts -- --ignored db_`

use crate::accounts::{self, Recorded};
use crate::intake::session;
use lifeguard::{LifeExecutor, SessionContext};
use rerp_crm_conversion::{
    manual_conversion_id, stripe_conversion_id, Address, Conversion, Customer, Reason, Screened,
    StripeRef, VERSION,
};
use uuid::Uuid;

fn customer(name: &str) -> Customer {
    Customer {
        name: name.into(),
        contact_name: Some("Ada".into()),
        email: "billing@example.com".into(),
        address: Address {
            line1: "1 High Street".into(),
            line2: None,
            city: "Sofia".into(),
            postal_code: Some("1000".into()),
            region: None,
            country: "BG".into(),
        },
        phone: Some("+359881234567".into()),
        tax_id: Some("BG123".into()),
    }
}

fn manual(lead: Uuid, name: &str) -> Screened {
    Screened {
        conversion: Conversion {
            version: VERSION,
            conversion_id: manual_conversion_id(lead),
            tenant_id: "pricewhisperer".into(),
            organization_id: Uuid::from_u128(9),
            reason: Reason::Order,
            reference: "PO-1".into(),
            funnel_lead_id: Some(lead),
            stripe: None,
            customer: customer(name),
            converted_by: Some("staff-1".into()),
            converted_at: "2026-10-23T00:00:00Z".parse().unwrap(),
        },
        signed_by: Some("k1".into()),
    }
}

fn stripe(customer_id: &str, session: &str) -> Screened {
    Screened {
        conversion: Conversion {
            version: VERSION,
            conversion_id: stripe_conversion_id(session),
            tenant_id: "pricewhisperer".into(),
            organization_id: Uuid::from_u128(9),
            reason: Reason::StripeCheckout,
            reference: session.into(),
            funnel_lead_id: None,
            stripe: Some(StripeRef {
                customer_id: customer_id.into(),
                checkout_session_id: session.into(),
            }),
            customer: customer("Stripe Buyer"),
            converted_by: None,
            converted_at: "2026-10-23T01:00:00Z".parse().unwrap(),
        },
        signed_by: None,
    }
}

fn run<T>(
    ctx: &SessionContext,
    f: impl FnOnce(&lifeguard::ExclusivePrimaryLifeExecutor<'_>) -> Result<T, lifeguard::LifeError>,
) -> Result<T, lifeguard::LifeError> {
    crate::database::db()
        .expect("database")
        .pool()
        .with_session_transaction(ctx, f)
}

#[test]
#[ignore]
fn db_conversions_create_once_link_and_stay_tenant_scoped() {
    let db = crate::database::db().expect("database");
    crate::schema::ensure(db).expect("schema");
    crate::schema::ensure(db).expect("schema is idempotent");
    let unique = Uuid::new_v4();
    let lead = Uuid::new_v4();
    let a = manual(lead, &format!("Acme {unique}"));
    let ctx = session(&a.conversion);

    let first = run(&ctx, |e| accounts::record(e, &a)).unwrap();
    let Recorded::Created(account) = first else {
        panic!("expected a new customer, got {first:?}")
    };
    assert_eq!(
        run(&ctx, |e| accounts::record(e, &a)).unwrap(),
        Recorded::Duplicate(account)
    );

    let cus = format!("cus_{}", unique.simple());
    let s1 = stripe(&cus, &format!("cs_{}_1", unique.simple()));
    let s2 = stripe(&cus, &format!("cs_{}_2", unique.simple()));
    let Recorded::Created(buyer) = run(&ctx, |e| accounts::record(e, &s1)).unwrap() else {
        panic!("expected new")
    };
    assert_eq!(
        run(&ctx, |e| accounts::record(e, &s2)).unwrap(),
        Recorded::Linked(buyer)
    );

    let got = run(&ctx, |e| accounts::get(e, account))
        .unwrap()
        .expect("customer");
    assert_eq!(got["name"], format!("Acme {unique}"));
    assert_eq!(got["country_code"], "BG");
    assert_eq!(got["origin"]["reason"], "order");
    assert_eq!(got["origin"]["funnel_lead_id"], lead.to_string());

    let listed = run(&ctx, |e| {
        accounts::list(e, Some(&unique.to_string()), None, 1, 20)
    })
    .unwrap();
    assert_eq!(listed["total"], 1);

    // Another tenant sees nothing (RLS).
    let mut other = ctx.clone();
    other.tenant_id = "someone-else".into();
    assert!(run(&other, |e| accounts::get(e, account))
        .unwrap()
        .is_none());
    assert_eq!(
        run(&other, |e| accounts::list(e, None, None, 1, 20)).unwrap()["total"],
        0
    );

    // Conversions are immutable.
    let id = a.conversion.conversion_id.to_string();
    let changed = run(&ctx, |e| {
        e.execute_values(
            "UPDATE crm_conversions SET reference = 'x' WHERE conversion_id = $1::text::uuid",
            &sea_query::Values(vec![sea_query::Value::from(id.clone())]),
        )
    });
    assert!(changed.is_err(), "conversion rows must not change");
}
