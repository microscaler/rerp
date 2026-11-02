//! Live-DB acceptance for the AR overlay migration (Epic F1a): tenant
//! isolation via FORCE ROW LEVEL SECURITY for all four tables, verified
//! through the app's own `rls_set_session` contract. Every negative case
//! asserts the *reason* psql failed (row-level security policy vs a named
//! foreign-key constraint), so a syntax slip or an unrelated violation can
//! never masquerade as the assertion passing.
//!
//! Requires the disposable database from
//! `microservices/accounting/scripts/setup-db.sh` (or the scratch reset
//! harness) with migrations 0001..0006 applied and DML granted to the
//! `rerp` role; `psql` must be on PATH. These live tests share one
//! database and must run serially: `cargo test -- --ignored
//! --test-threads=1`.

use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

/// Distinguishes the SQL files written by concurrent psql calls.
static SEQ: AtomicU64 = AtomicU64::new(0);

fn env_or(key: &str, fallback: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| fallback.to_string())
}

/// Run-unique fixture ids: `{prefix}{slot}{run6}` first group, so reruns
/// and parallel processes never collide on primary keys.
fn uid(prefix: &str, slot: &str, run6: &str) -> String {
    format!("{prefix}{slot}{run6}-0000-4000-8000-000000000000")
}

fn run_hex() -> String {
    // Random per process (not just pid: pids get reused, and the fixture
    // rows persist across runs — a pid collision would collide on the
    // legal-entity PK before the RLS assertions run). Stable within a run.
    // 24 bits of entropy is ample for the handful of concurrent runs a
    // disposable database ever sees.
    static TAG: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    TAG.get_or_init(|| {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        let mix = (now ^ ((std::process::id() as u64) << 32)).wrapping_mul(0x9E37_79B9_7F4A_7C15)
            >> 24
            & 0x00FF_FFFF;
        format!("{mix:06x}")
    })
    .clone()
}

/// Runs one psql session against the disposable DB. `session_sql` runs
/// first (tenant context or its deliberate absence), then `body_sql`.
/// Returns (success, stdout, stderr).
fn run_psql(session_sql: &str, body_sql: &str) -> (bool, String, String) {
    let seq = SEQ.fetch_add(1, Ordering::SeqCst);
    let mut path = std::env::temp_dir();
    path.push(format!("rerp-ar-rls-{}-{seq}.sql", std::process::id()));
    std::fs::write(&path, format!("{session_sql}\n{body_sql}")).expect("write sql");
    let output = Command::new("psql")
        .env("PGHOST", env_or("DB_HOST", "localhost"))
        .env("PGPORT", env_or("DB_PORT", "5432"))
        .env("PGUSER", env_or("DB_USER", "rerp"))
        .env("PGPASSWORD", env_or("DB_PASS", ""))
        .env("PGDATABASE", env_or("DB_NAME", "rerp"))
        .args(["-v", "ON_ERROR_STOP=1", "-q", "-At", "-f"])
        .arg(&path)
        .output()
        .expect("psql on PATH");
    let _ = std::fs::remove_file(&path);
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).trim().to_string(),
        String::from_utf8_lossy(&output.stderr).trim().to_string(),
    )
}

/// psql must succeed; returns stdout.
fn psql(session_sql: &str, body_sql: &str) -> String {
    let (ok, stdout, stderr) = run_psql(session_sql, body_sql);
    if !ok {
        // Failure context only: psql's stderr already carries the failing
        // statement's file and line; the SQL text itself is not echoed.
        panic!("psql failed unexpectedly: stderr: {stderr}");
    }
    stdout
}

/// psql must FAIL, and for exactly `needle`: the combined output must
/// contain the rejection reason (RLS policy text or a named constraint).
/// A syntax error or unrelated violation fails this assertion.
fn psql_fails_with(session_sql: &str, body_sql: &str, needle: &str) {
    let (ok, stdout, stderr) = run_psql(session_sql, body_sql);
    if ok {
        panic!("psql unexpectedly succeeded: stdout: {stdout}");
    }
    assert!(
        stderr.contains(needle),
        "expected rejection containing {needle:?}, got: {stderr}"
    );
}

/// Full per-tenant fixture chain (legal entity, open period, TWO posted
/// documents) plus rows in each of the four AR tables, including a SECOND
/// payment — spare anchors so no-session write probes hit the RLS check
/// instead of a unique-key or foreign-key conflict. Everything runs
/// inside the tenant's own RLS session, exactly like the application.
fn seed_tenant(tenant: &str, prefix: &str, run: &str) -> String {
    let le = uid(prefix, "1", run);
    let subject = uid(prefix, "2", run);
    let org = uid(prefix, "3", run);
    let customer = uid(prefix, "4", run);
    let document2 = uid(prefix, "7", run);
    let period = uid(prefix, "8", run);
    let payment2 = uid(prefix, "c", run);
    let document = uid(prefix, "d", run);
    let overlay = uid(prefix, "e", run);
    let payment = uid(prefix, "f", run);
    format!(
        "BEGIN;
SELECT public.rls_set_session('{tenant}', '{subject}'::uuid,
    '{org}'::uuid,
    'ar-rls-seed', '[\"billing\"]'::jsonb,
    '[\"accounting:invoice:write\"]'::jsonb, 'service', 'shipper');
INSERT INTO rerp.accounting_legal_entities
    (id, tenant_id, organization_id, legal_name, base_currency, is_active, created_at, updated_at)
VALUES ('{le}', '{tenant}', '{org}', 'AR RLS {tenant}', 'GBP', true, now(), now());
INSERT INTO rerp.accounting_fiscal_periods
    (id, tenant_id, legal_entity_id, name, start_date, end_date, state, created_at, updated_at)
VALUES ('{period}', '{tenant}', '{le}', 'FY2026', '2026-01-01', '2026-12-31',
    'OPEN', now(), now());
INSERT INTO rerp.accounting_posted_documents
    (id, tenant_id, legal_entity_id, fiscal_period_id, document_number, document_type,
     customer_id, source_system, source_type, source_id, document_date, due_date,
     currency_code, rounding_minor_units, subtotal, discount_amount, tax_amount,
     total_amount, posted_at, posted_by)
VALUES ('{document}', '{tenant}', '{le}', '{period}', '{tenant}-INV-1',
    'CUSTOMER_INVOICE', '{customer}', 'ar-rls-test', 'fixture', '{tenant}',
    '2026-06-01', '2026-07-01', 'GBP', 2, 100.000000, 0, 20.000000, 120.000000,
    now(), '{subject}'::uuid),
    ('{document2}', '{tenant}', '{le}', '{period}', '{tenant}-INV-2',
    'CUSTOMER_INVOICE', '{customer}', 'ar-rls-test', 'fixture2', '{tenant}',
    '2026-06-02', '2026-07-02', 'GBP', 2, 100.000000, 0, 20.000000, 120.000000,
    now(), '{subject}'::uuid);
INSERT INTO rerp.customer_invoices
    (id, tenant_id, legal_entity_id, invoice_id, customer_id, original_amount,
     outstanding_amount, currency_code, due_date, terms, updated_at)
VALUES ('{overlay}', '{tenant}', '{le}', '{document}', '{customer}',
    120.000000, 120.000000, 'GBP', '2026-07-01', 'NET_30', now());
INSERT INTO rerp.ar_payments
    (id, tenant_id, legal_entity_id, customer_id, payment_number, payment_date,
     payment_amount, currency_code, payment_method, status, updated_at)
VALUES ('{payment}', '{tenant}', '{le}', '{customer}', '{tenant}-PMT-1',
    '2026-06-15', 60.000000, 'GBP', 'WIRE', 'DRAFT', now()),
    ('{payment2}', '{tenant}', '{le}', '{customer}', '{tenant}-PMT-2',
    '2026-06-16', 60.000000, 'GBP', 'WIRE', 'DRAFT', now());
INSERT INTO rerp.ar_payment_applications
    (id, tenant_id, legal_entity_id, payment_id, invoice_id, applied_amount)
VALUES ('{overlay}', '{tenant}', '{le}', '{payment}', '{overlay}', 60.000000);
INSERT INTO rerp.ar_agings
    (id, tenant_id, legal_entity_id, customer_id, aging_date, current,
     total_outstanding, currency_code)
VALUES ('{overlay}', '{tenant}', '{le}', '{customer}', '2026-07-01', 120.000000,
    120.000000, 'GBP');
COMMIT;"
    )
}

fn session(tenant: &str, prefix: &str, run: &str) -> String {
    format!(
        "SELECT public.rls_set_session('{tenant}', '{}'::uuid,
    '{}'::uuid, 'ar-rls-read',
    '[\"billing\"]'::jsonb, '[]'::jsonb, 'service', 'shipper');",
        uid(prefix, "2", run),
        uid(prefix, "3", run),
    )
}

const COUNT_ALL: &str = "SELECT
    (SELECT count(*) FROM rerp.customer_invoices) || '/' ||
    (SELECT count(*) FROM rerp.ar_payments) || '/' ||
    (SELECT count(*) FROM rerp.ar_payment_applications) || '/' ||
    (SELECT count(*) FROM rerp.ar_agings);";

const SP: &str = "SET search_path TO rerp, public;";

#[test]
#[ignore = "requires the disposable accounting DB (migrations 0001..0006) and psql on PATH"]
fn ar_overlay_tenant_isolation() {
    // Two independent tenants, each seeded entirely through its own RLS
    // session (writes only succeed via WITH CHECK on the own tenant).
    let run = run_hex();
    let tenant_a = format!("ar-rls-a-{run}");
    let tenant_b = format!("ar-rls-b-{run}");
    psql(SP, &seed_tenant(&tenant_a, "a", &run));
    psql(SP, &seed_tenant(&tenant_b, "b", &run));

    // Each tenant session sees exactly its own rows in every table
    // (1 overlay, 2 payments, 1 application, 1 aging). The session GUC
    // is transaction-local, so read inside one transaction.
    let seen_a = psql(
        SP,
        &format!(
            "BEGIN;\n{}\n{COUNT_ALL}\nCOMMIT;",
            session(&tenant_a, "a", &run)
        ),
    );
    let seen_b = psql(
        SP,
        &format!(
            "BEGIN;\n{}\n{COUNT_ALL}\nCOMMIT;",
            session(&tenant_b, "b", &run)
        ),
    );
    assert_eq!(seen_a, "1/2/1/1", "tenant A sees only its own rows");
    assert_eq!(seen_b, "1/2/1/1", "tenant B sees only its own rows");

    // No tenant context: nothing is visible...
    let sightless = psql(SP, COUNT_ALL);
    assert_eq!(sightless, "0/0/0/0", "no session -> no rows");

    // ...and no write to ANY of the four tables is permitted: each probe
    // uses tenant A's real seeded anchors (legal entity, customer, spare
    // document/payment) so the ONLY possible rejection is the RLS
    // WITH CHECK / USING — asserted by requiring the policy error text.
    let le_a = uid("a", "1", &run);
    let cust_a = uid("a", "4", &run);
    psql_fails_with(
        SP,
        &format!(
            "INSERT INTO rerp.customer_invoices
                (id, tenant_id, legal_entity_id, invoice_id, customer_id,
                 original_amount, outstanding_amount, currency_code, due_date,
                 terms, updated_at)
             VALUES ('{oi}', '{tenant_a}', '{le_a}', '{d2}', '{cust_a}',
                 1, 1, 'GBP', '2026-07-15', 'NET_30', now());",
            oi = uid("a", "5", &run),
            d2 = uid("a", "7", &run),
        ),
        "row-level security",
    );
    psql_fails_with(
        SP,
        &format!(
            "INSERT INTO rerp.ar_payments
                (id, tenant_id, legal_entity_id, customer_id, payment_number,
                 payment_date, payment_amount, currency_code, payment_method,
                 status, updated_at)
             VALUES ('{pi}', '{tenant_a}', '{le_a}', '{cust_a}',
                 '{tenant_a}-PMT-PROBE', '2026-06-20', 1, 'GBP', 'WIRE',
                 'DRAFT', now());",
            pi = uid("a", "6", &run),
        ),
        "row-level security",
    );
    psql_fails_with(
        SP,
        &format!(
            "INSERT INTO rerp.ar_payment_applications
                (id, tenant_id, legal_entity_id, payment_id, invoice_id,
                 applied_amount)
             VALUES ('{ai}', '{tenant_a}', '{le_a}', '{p2}', '{oi}', 1);",
            ai = uid("a", "b", &run),
            p2 = uid("a", "c", &run),
            oi = uid("a", "e", &run),
        ),
        "row-level security",
    );
    psql_fails_with(
        SP,
        &format!(
            "INSERT INTO rerp.ar_agings
                (id, tenant_id, legal_entity_id, customer_id, aging_date,
                 current, total_outstanding, currency_code)
             VALUES ('{gi}', '{tenant_a}', '{le_a}', '{cust_a}', '2026-08-01',
                 1, 1, 'GBP');",
            gi = uid("a", "0", &run),
        ),
        "row-level security",
    );
}

#[test]
#[ignore = "requires the disposable accounting DB (migrations 0001..0006) and psql on PATH"]
fn ar_overlay_composite_fk_scopes_are_tenant_consistent() {
    // Cross-tenant anchors must be impossible even with a valid own-tenant
    // session, and must fail on the NAMED composite foreign key — not on
    // anything else. Both tenants are seeded inside this test so the
    // foreign anchors genuinely exist (RLS passes, FK is the only reject).
    let run = run_hex();
    let tenant = format!("ar-rls-fk-{run}");
    let foreign = format!("ar-rls-fx-{run}");
    psql(SP, &seed_tenant(&tenant, "c", &run));
    psql(SP, &seed_tenant(&foreign, "e", &run));
    let foreign_document = uid("e", "d", &run);
    let foreign_entity = uid("e", "1", &run);

    // (tenant, legal_entity, invoice) -> posted_documents of ANOTHER tenant.
    psql_fails_with(
        SP,
        &format!(
            "BEGIN;
{}
INSERT INTO rerp.customer_invoices
    (id, tenant_id, legal_entity_id, invoice_id, customer_id, original_amount,
     outstanding_amount, currency_code, due_date, terms, updated_at)
VALUES ('{}', '{}',
        '{}',
        '{}',
        '{}', 1, 1, 'GBP',
        '2026-07-01', 'NET_30', now());
COMMIT;",
            session(&tenant, "c", &run),
            uid("c", "5", &run),
            tenant,
            uid("c", "1", &run),
            foreign_document,
            uid("c", "4", &run),
        ),
        "customer_invoices_document_scope_fk",
    );

    // (tenant, legal_entity) -> accounting_legal_entities of ANOTHER tenant.
    psql_fails_with(
        SP,
        &format!(
            "BEGIN;
{}
INSERT INTO rerp.ar_payments
    (id, tenant_id, legal_entity_id, customer_id, payment_number, payment_date,
     payment_amount, currency_code, payment_method, status, updated_at)
VALUES ('{}', '{}',
        '{}',
        '{}', '{}-PMT-X',
        '2026-06-15', 1, 'GBP', 'WIRE', 'DRAFT', now());
COMMIT;",
            session(&tenant, "c", &run),
            uid("c", "6", &run),
            tenant,
            foreign_entity,
            uid("c", "4", &run),
            tenant,
        ),
        "ar_payments_legal_entity_scope_fk",
    );
}
