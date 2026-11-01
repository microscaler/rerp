//! E3 — live HTTPS consumer proof (BUILDOUT goal 6 completion).
//!
//! Production topology, faithfully: the REAL invoice binary runs as its own
//! process (plaintext HTTP/1.1, TLS terminated at the edge), a test-only
//! rustls terminator (rcgen leaf for `localhost`) forwards decrypted bytes to
//! it, a mock identity endpoint serves the HS256 JWKS the binary's
//! config-registered `JwksBearerProvider` validates, and the consumer drives
//! the generated contract types over `may_minihttp`'s rustls client — the
//! exact client production consumers use.
//!
//! Journey over one HTTPS URL: post, idempotent replay, changed-payload
//! conflict, typed retrieve, full credit note, additional-receivable debit
//! note, original-document immutability.
//!
//! Requires a disposable PostgreSQL database with the RERP foundation
//! migrations applied and `psql` on PATH (same requirement class as the
//! `tests/sql` acceptance harness); the fixture tenant `e2e-https` is seeded
//! here via the app's own `rls_set_session` contract. Environment: DB_HOST /
//! DB_PORT / DB_USER / DB_PASS / DB_NAME as for the service binary.
//!
//! Run: `cargo test -p rerp_accounting_invoice --test e2e_https -- --ignored`

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use chrono::Datelike as _;
use http_legacy::Method;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use may::coroutine::Builder as CoroutineBuilder;
use may_minihttp::client::Client;
use rcgen::generate_simple_self_signed;
use rerp_accounting_invoice_gen::handlers::credit_customer_invoice;
use rerp_accounting_invoice_gen::handlers::debit_customer_invoice;
use rerp_accounting_invoice_gen::handlers::get_customer_invoice;
use rerp_accounting_invoice_gen::handlers::post_customer_invoice;
use rerp_accounting_invoice_gen::handlers::types::{
    CreditCustomerInvoiceRequest, CustomerInvoiceLine, DebitCustomerInvoiceRequest, DebitLineInput,
    SourceReference,
};
use serde_json::{json, Value};

const TENANT: &str = "e2e-https";
const ISSUER: &str = "https://identity.e2e.local";
const AUDIENCE: &str = "rerp-accounting";
const KID: &str = "e2e-kid";
const SECRET: &[u8] = b"e2e-https-shared-secret-0123456789";

const LEGAL_ENTITY: &str = "e1000000-0000-4000-8000-0000000000e1";
const ORGANIZATION: &str = "e2000000-0000-4000-8000-0000000000e2";
const SUBJECT: &str = "e3000000-0000-4000-8000-0000000000e3";
const CUSTOMER: &str = "e4000000-0000-4000-8000-0000000000e4";

fn env_or(key: &str, fallback: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| fallback.to_string())
}

/// Sesame-shaped HS256 access token; claims match what
/// `sesame_idam_client::parse_validated_claims` demands.
fn mint_token() -> String {
    let now = chrono::Utc::now().timestamp();
    let claims = json!({
        "iss": ISSUER,
        "aud": AUDIENCE,
        "iat": now,
        "exp": now + 3600,
        "sub": SUBJECT,
        "user_id": SUBJECT,
        "sid": "e2e-https-session",
        "tenant_id": TENANT,
        "org_id": ORGANIZATION,
        "user_type": "service",
        "org_type": "shipper",
        "https://sesame-idam.dev/claims": {
            "tenant": TENANT,
            "roles": ["billing"],
            "permissions": ["accounting:invoice:write"]
        }
    });
    let mut header = Header::new(Algorithm::HS256);
    header.kid = Some(KID.to_string());
    // JwksBearerProvider enforces the Sesame access-token type convention.
    header.typ = Some("at+jwt".to_string());
    encode(&header, &claims, &EncodingKey::from_secret(SECRET)).expect("mint HS256 token")
}

/// Minimal JWKS endpoint: one oct (HS256) key, HTTP/1.1 with Connection:
/// close. Plain std thread — the provider fetches during request handling.
fn spawn_mock_jwks() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock JWKS");
    let port = listener.local_addr().expect("mock JWKS port").port();
    let body = json!({
        "keys": [{ "kty": "oct", "alg": "HS256", "kid": KID, "k": URL_SAFE_NO_PAD.encode(SECRET) }]
    })
    .to_string();
    thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(mut stream) = stream else { continue };
            let mut request = Vec::new();
            let mut byte = [0_u8; 1];
            while !request.ends_with(b"\r\n\r\n") && stream.read(&mut byte).unwrap_or(0) == 1 {
                request.push(byte[0]);
            }
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            let _ = stream.write_all(response.as_bytes());
            let _ = stream.flush();
        }
    });
    port
}

/// Test-only TLS edge: rustls server config for `localhost` plus the
/// matching client root store — the same rustls stack consumers use.
fn build_tls_configs() -> (rustls::ServerConfig, Arc<rustls::ClientConfig>) {
    use rcgen::CertifiedKey;
    use rustls::pki_types::PrivatePkcs8KeyDer;
    use rustls::{ClientConfig, RootCertStore, ServerConfig};

    let CertifiedKey { cert, signing_key } =
        generate_simple_self_signed(vec!["localhost".to_owned()]).expect("rcgen leaf");
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let server_config = ServerConfig::builder_with_provider(provider.clone())
        .with_safe_default_protocol_versions()
        .expect("server tls protocol versions")
        .with_no_client_auth()
        .with_single_cert(
            vec![cert.der().clone()],
            PrivatePkcs8KeyDer::from(signing_key.serialize_der()).into(),
        )
        .expect("server tls config");
    let mut roots = RootCertStore::empty();
    roots.add(cert.der().clone()).expect("trust rcgen root");
    let client_config = ClientConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()
        .expect("client tls protocol versions")
        .with_root_certificates(roots)
        .with_no_client_auth();
    (server_config, Arc::new(client_config))
}

/// Forwarding pump for one edge connection (decrypt → service, encrypt ←
/// service). Read timeouts surface as `WouldBlock`; EOF on both sides or
/// ~10 s of no progress closes the connection.
fn pump_tls_connection(
    server_config: Arc<rustls::ServerConfig>,
    client: TcpStream,
    service: SocketAddr,
) {
    use rustls::{ServerConnection, StreamOwned};

    let Ok(mut inner) = TcpStream::connect(service) else {
        return;
    };
    if client
        .set_read_timeout(Some(Duration::from_millis(250)))
        .and(inner.set_read_timeout(Some(Duration::from_millis(250))))
        .is_err()
    {
        return;
    }
    let Ok(connection) = ServerConnection::new(server_config) else {
        return;
    };
    let mut tls = StreamOwned::new(connection, client);
    let mut to_inner = [0_u8; 8192];
    let mut to_client = [0_u8; 8192];
    let mut client_eof = false;
    let mut inner_eof = false;
    let mut idle_ticks = 0_u32;
    loop {
        let mut progress = false;
        if !client_eof {
            match tls.read(&mut to_inner) {
                Ok(0) => client_eof = true,
                Ok(n) => progress = inner.write_all(&to_inner[..n]).is_ok(),
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(_) => client_eof = true,
            }
        }
        if !inner_eof {
            match inner.read(&mut to_client) {
                Ok(0) => inner_eof = true,
                Ok(n) => {
                    if tls.write_all(&to_client[..n]).is_ok() && tls.flush().is_ok() {
                        progress = true;
                    }
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(_) => inner_eof = true,
            }
        }
        if client_eof && inner_eof {
            break;
        }
        idle_ticks = if progress { 0 } else { idle_ticks + 1 };
        if idle_ticks > 40 {
            break;
        }
        if !progress {
            thread::sleep(Duration::from_millis(250));
        }
    }
}

/// Accept loop: terminates TLS and pumps each connection to the service.
fn spawn_tls_edge(server_config: rustls::ServerConfig, service: SocketAddr) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind TLS edge");
    let port = listener.local_addr().expect("TLS edge port").port();
    let server_config = Arc::new(server_config);
    thread::spawn(move || {
        for accepted in listener.incoming() {
            let Ok(client) = accepted else { continue };
            let config = Arc::clone(&server_config);
            thread::spawn(move || pump_tls_connection(config, client, service));
        }
    });
    port
}

/// Seed the fixture tenant through the app's own RLS session contract.
/// Idempotent (ON CONFLICT DO NOTHING) so reruns are safe.
fn seed_fixture() {
    let year = chrono::Utc::now().date_naive().year();
    let sql = format!(
        "BEGIN;
SELECT public.rls_set_session('{tenant}', '{subject}'::uuid, '{org}'::uuid,
    'e2e-https-seed', '[\"billing\"]'::jsonb,
    '[\"accounting:invoice:write\"]'::jsonb, 'service', 'shipper');
INSERT INTO rerp.accounting_legal_entities
    (id, tenant_id, organization_id, legal_name, base_currency, is_active, created_at, updated_at)
VALUES ('{le}', '{tenant}', '{org}', 'E2E HTTPS Test Legal Entity', 'GBP', true, now(), now())
ON CONFLICT (id) DO NOTHING;
INSERT INTO rerp.accounting_fiscal_periods
    (id, tenant_id, legal_entity_id, name, start_date, end_date, state, created_at, updated_at)
VALUES ('e8000000-0000-4000-8000-0000000000e8', '{tenant}', '{le}', 'FY{year}',
        '{year}-01-01', '{year}-12-31', 'OPEN', now(), now())
ON CONFLICT (id) DO NOTHING;
INSERT INTO rerp.accounting_accounts
    (id, tenant_id, legal_entity_id, code, name, account_type, normal_side, control_role, currency_code, is_active, created_at, updated_at)
VALUES
    ('e5000000-0000-4000-8000-0000000000e5', '{tenant}', '{le}', 'E2E-AR',  'E2E AR',       'ASSET',    'DEBIT',  'ACCOUNTS_RECEIVABLE', 'GBP', true, now(), now()),
    ('e6000000-0000-4000-8000-0000000000e6', '{tenant}', '{le}', 'E2E-REV', 'E2E Revenue',  'REVENUE',  'CREDIT', 'REVENUE',             'GBP', true, now(), now()),
    ('e7000000-0000-4000-8000-0000000000e7', '{tenant}', '{le}', 'E2E-VAT', 'E2E VAT',      'LIABILITY','CREDIT', 'TAX_PAYABLE',         'GBP', true, now(), now())
ON CONFLICT (id) DO NOTHING;
COMMIT;",
        tenant = TENANT,
        subject = SUBJECT,
        org = ORGANIZATION,
        le = LEGAL_ENTITY,
        year = year
    );
    let mut script = std::env::temp_dir();
    script.push(format!("rerp-e2e-seed-{year}.sql"));
    std::fs::write(&script, sql).expect("write seed SQL");
    let status = Command::new("psql")
        .env("PGHOST", env_or("DB_HOST", "localhost"))
        .env("PGPORT", env_or("DB_PORT", "5432"))
        .env("PGUSER", env_or("DB_USER", "rerp"))
        .env("PGPASSWORD", env_or("DB_PASS", ""))
        .env("PGDATABASE", env_or("DB_NAME", "rerp"))
        .args(["-v", "ON_ERROR_STOP=1", "-q", "-f"])
        .arg(&script)
        .status()
        .expect("psql on PATH");
    assert!(status.success(), "fixture seed failed: {script:?}");
}

/// Boots the real invoice binary against the disposable database with
/// bearerAuth bound to the mock JWKS endpoint. Returns the child and its
/// plaintext port; the child is killed on drop.
///
/// Binary resolution: `RERP_E2E_BIN` overrides; otherwise the binary cargo
/// built for this test (`CARGO_BIN_EXE_invoice`). On hosts where the may +
/// generator coroutine runtime misbehaves in debug builds (observed on
/// rustc 1.99 Linux: finished request coroutines trip generator's stack
/// guard and abort the process, release unaffected), point the override at
/// a `cargo build --release -p rerp_accounting_invoice` binary.
fn spawn_service(jwks_port: u16) -> (ServiceChild, u16) {
    let probe = TcpListener::bind("127.0.0.1:0").expect("reserve service port");
    let port = probe.local_addr().expect("service port").port();
    drop(probe);
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let spec = manifest.join("../gen/doc/openapi.yaml");
    let binary =
        std::env::var("RERP_E2E_BIN").unwrap_or_else(|_| env!("CARGO_BIN_EXE_invoice").to_string());
    let child = Command::new(&binary)
        .arg("--spec")
        .arg(&spec)
        .arg("--config")
        .arg(write_service_config(port, jwks_port))
        .env("BRRTR_LOCAL", "1")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap_or_else(|error| panic!("spawn {binary}: {error}"));
    (ServiceChild { child: Some(child) }, port)
}

fn write_service_config(port: u16, jwks_port: u16) -> PathBuf {
    let config_yaml = format!(
        "port: {port}\nsecurity:\n  jwks:\n    bearerAuth:\n      jwks_url: http://127.0.0.1:{jwks_port}/.well-known/jwks.json\n      iss: {ISSUER}\n      aud: {AUDIENCE}\n"
    );
    let mut config_path = std::env::temp_dir();
    config_path.push(format!("rerp-e2e-service-{port}.yaml"));
    std::fs::write(&config_path, config_yaml).expect("write service config");
    config_path
}

struct ServiceChild {
    child: Option<std::process::Child>,
}

impl Drop for ServiceChild {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

/// One typed consumer request over the HTTPS edge. `connection: close` keeps
/// the connection lifecycle deterministic end to end.
fn consumer_send<T: serde::Serialize>(
    client: &Client,
    method: Method,
    url: &str,
    token: &str,
    body: Option<&T>,
) -> Result<(u16, Value), String> {
    let mut builder = client
        .request(method, url)
        .map_err(|error| format!("build request {url}: {error}"))?
        .header_str("authorization", &format!("Bearer {token}"))
        .map_err(|error| format!("auth header: {error}"))?
        .header_str("connection", "close")
        .map_err(|error| format!("connection header: {error}"))?;
    if let Some(body) = body {
        let payload =
            serde_json::to_string(body).map_err(|error| format!("serialise body: {error}"))?;
        builder = builder
            .header_str("content-type", "application/json")
            .map_err(|error| format!("content-type header: {error}"))?
            .body(payload.into_bytes());
    }
    let response = builder
        .send()
        .map_err(|error| format!("send {url}: {error}"))?;
    let status = response.status().as_u16();
    if response.body().is_empty() {
        return Ok((status, Value::Null));
    }
    let value: Value = serde_json::from_slice(response.body())
        .map_err(|error| format!("response body from {url}: {error}"))?;
    Ok((status, value))
}

fn post_body(date: &str, key: &str, unit_price: &str) -> post_customer_invoice::Request {
    post_customer_invoice::Request {
        currency_code: "GBP".to_string(),
        customer_id: CUSTOMER.to_string(),
        due_date: date.to_string(),
        idempotency_key: key.to_string(),
        invoice_date: date.to_string(),
        lines: vec![CustomerInvoiceLine {
            description: "E2E HTTPS delivery".to_string(),
            discount_percent: Some("0".to_string()),
            quantity: "1".to_string(),
            tax: Some(json!({ "code": "VAT20", "rate_percent": "20" })),
            unit_price: unit_price.to_string(),
        }],
        rounding_minor_units: Some(2),
        source: SourceReference {
            system: "e2e-https".to_string(),
            resource_type: "https_probe".to_string(),
            resource_id: format!("https-probe-{key}"),
        },
    }
}

/// Money strings compare numerically: POST envelopes are 2dp ("120.00"),
/// retrieve reads the DB numeric scale ("120.000000"); both are valid.
fn money_eq(a: &str, b: &str) -> bool {
    let norm = |s: &str| s.trim_end_matches('0').trim_end_matches('.').to_string();
    norm(a) == norm(b)
}

/// Full consumer journey over one HTTPS base URL, typed against the generated
/// contract. Returns observed status codes plus the original total after the
/// credit and debit legs (immutability check).
fn run_consumer_flow(
    base: &str,
    client_config: Arc<rustls::ClientConfig>,
    token: &str,
    date: &str,
) -> Result<(u16, u16, u16, u16, u16, String), String> {
    let client = Client::builder()
        .tls_config(client_config)
        .request_timeout(Duration::from_secs(20))
        .build()
        .map_err(|error| format!("build consumer client: {error}"))?;
    let invoices = format!("{base}/v1/customer-invoices");

    // 0. Readiness: retry until the edge answers anything at all (the pump
    //    refuses upstream connections until the binary is listening).
    let mut ready = false;
    for _ in 0..120 {
        match consumer_send::<()>(&client, Method::GET, &invoices, token, None) {
            Ok((_, _)) => {
                ready = true;
                break;
            }
            Err(_) => thread::sleep(Duration::from_millis(250)),
        }
    }
    if !ready {
        return Err("service never became reachable through the TLS edge".to_string());
    }

    // 1. Post over HTTPS — rustls handshake, JWKS bearer auth, validation.
    let (post_status, post_json) = consumer_send(
        &client,
        Method::POST,
        &invoices,
        token,
        Some(&post_body(date, "e2e-post-1", "100.00")),
    )?;
    if post_status != 201 {
        return Err(format!("post returned {post_status}: {post_json}"));
    }
    let created: post_customer_invoice::Response =
        serde_json::from_value(post_json).map_err(|error| format!("post envelope: {error}"))?;
    if !money_eq(&created.invoice.total_amount, "120.00") {
        return Err(format!(
            "expected 120.00, got {}",
            created.invoice.total_amount
        ));
    }
    let invoice_id = created.invoice.id.clone();

    // 2. Identical replay answers 200 with the same document.
    let (replay_status, replay_json) = consumer_send(
        &client,
        Method::POST,
        &invoices,
        token,
        Some(&post_body(date, "e2e-post-1", "100.00")),
    )?;
    let replay: post_customer_invoice::Response =
        serde_json::from_value(replay_json).map_err(|error| format!("replay envelope: {error}"))?;
    if replay.invoice.id != invoice_id {
        return Err("idempotent replay returned a different document".to_string());
    }

    // 3. Changed payload on the same key is a 409 conflict.
    let (conflict_status, _) = consumer_send(
        &client,
        Method::POST,
        &invoices,
        token,
        Some(&post_body(date, "e2e-post-1", "101.00")),
    )?;

    // 4. Typed retrieve of the original.
    let (get_status, get_json) = consumer_send::<()>(
        &client,
        Method::GET,
        &format!("{invoices}/{invoice_id}"),
        token,
        None,
    )?;
    if get_status != 200 {
        return Err(format!("retrieve returned {get_status}"));
    }
    let retrieved: get_customer_invoice::Response =
        serde_json::from_value(get_json).map_err(|error| format!("retrieve envelope: {error}"))?;
    if !money_eq(&retrieved.total_amount, "120.00") {
        return Err(format!("retrieve total {}", retrieved.total_amount));
    }

    // 5. Full credit note against the original.
    let credit = CreditCustomerInvoiceRequest {
        idempotency_key: "e2e-credit-1".to_string(),
        lines: None,
        reason: "E2E HTTPS cancellation".to_string(),
        source: SourceReference {
            system: "e2e-https".to_string(),
            resource_type: "https_probe_credit".to_string(),
            resource_id: "https-probe-credit-1".to_string(),
        },
    };
    let (credit_status, credit_json) = consumer_send(
        &client,
        Method::POST,
        &format!("{invoices}/{invoice_id}/credit-notes"),
        token,
        Some(&credit),
    )?;
    let credit: credit_customer_invoice::Response =
        serde_json::from_value(credit_json).map_err(|error| format!("credit envelope: {error}"))?;
    if credit.invoice.document_type != "CUSTOMER_CREDIT_NOTE" {
        return Err(format!("credit type {}", credit.invoice.document_type));
    }

    // 6. Additional-receivable debit note (uncapped by design: 2 x 100 + VAT).
    let debit = DebitCustomerInvoiceRequest {
        idempotency_key: "e2e-debit-1".to_string(),
        lines: vec![DebitLineInput {
            original_line_number: 1,
            quantity: "2.000000".to_string(),
        }],
        reason: "E2E HTTPS surcharge".to_string(),
        source: SourceReference {
            system: "e2e-https".to_string(),
            resource_type: "https_probe_debit".to_string(),
            resource_id: "https-probe-debit-1".to_string(),
        },
    };
    let (debit_status, debit_json) = consumer_send(
        &client,
        Method::POST,
        &format!("{invoices}/{invoice_id}/debit-notes"),
        token,
        Some(&debit),
    )?;
    let debit: debit_customer_invoice::Response =
        serde_json::from_value(debit_json).map_err(|error| format!("debit envelope: {error}"))?;
    if debit.invoice.document_type != "CUSTOMER_DEBIT_NOTE" {
        return Err(format!("debit type {}", debit.invoice.document_type));
    }
    if !debit.invoice.document_number.starts_with("DBN-") {
        return Err(format!("debit number {}", debit.invoice.document_number));
    }
    if !money_eq(&debit.invoice.total_amount, "240.00") {
        return Err(format!("debit total {}", debit.invoice.total_amount));
    }

    // 7. The original is untouched by credit and debit (immutability).
    let (_, original_json) = consumer_send::<()>(
        &client,
        Method::GET,
        &format!("{invoices}/{invoice_id}"),
        token,
        None,
    )?;
    let original: get_customer_invoice::Response =
        serde_json::from_value(original_json).map_err(|error| format!("re-retrieve: {error}"))?;
    Ok((
        post_status,
        replay_status,
        conflict_status,
        credit_status,
        debit_status,
        original.total_amount,
    ))
}

#[test]
#[ignore = "requires a disposable PostgreSQL database (with foundation migrations), psql on PATH, and the built invoice binary"]
fn live_https_consumer_full_journey() {
    seed_fixture();
    let jwks_port = spawn_mock_jwks();
    let (server_config, client_config) = build_tls_configs();
    let (service, service_port) = spawn_service(jwks_port);
    let edge_port = spawn_tls_edge(
        server_config,
        SocketAddr::from(([127, 0, 0, 1], service_port)),
    );

    let token = mint_token();
    let date = chrono::Utc::now().date_naive().to_string();
    let base = format!("https://localhost:{edge_port}");

    // The may_minihttp client performs coroutine I/O: run the journey inside
    // one coroutine with a client-sized stack (mirrors the crate's own tests).
    let outcome = unsafe {
        CoroutineBuilder::new()
            .stack_size(2 * 1024 * 1024)
            .spawn(move || run_consumer_flow(&base, client_config, &token, &date))
    }
    .expect("spawn consumer coroutine")
    .join()
    .expect("consumer coroutine panicked");
    drop(service);

    let (post_status, replay_status, conflict_status, credit_status, debit_status, original_total) =
        outcome.expect("HTTPS consumer journey");
    assert_eq!(post_status, 201, "post over HTTPS");
    assert_eq!(replay_status, 200, "idempotent replay over HTTPS");
    assert_eq!(conflict_status, 409, "changed payload conflict over HTTPS");
    assert_eq!(credit_status, 201, "credit note over HTTPS");
    assert_eq!(debit_status, 201, "debit note over HTTPS");
    assert!(
        money_eq(&original_total, "120.00"),
        "original stays immutable, got {original_total}"
    );
}
