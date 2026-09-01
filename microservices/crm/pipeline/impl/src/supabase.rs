//! Supabase marketing-DB access over PostgREST.
//!
//! The just-enough CRM reads the PriceWhisperer marketing database (Supabase
//! project `marketing`) directly over its REST API. Leads are a live view of
//! two website tables — `email_captures` (waiting-list signups) and
//! `contact_messages` (contact form) — joined with `email_addresses`,
//! `companies` and `plans`. Triage state (stage + note) is the one thing the
//! CRM owns: the `crm_lead_state` table, keyed by the source row's UUID.
//!
//! All HTTP goes through the coroutine-native `may_minihttp` client (rustls);
//! tokio-based clients cannot run inside a `may` service. The service_role
//! key comes from the environment (k8s Secret) and never reaches a browser.

use http_legacy::Method;
use may_minihttp::client::{Client, RedirectPolicy};
use rerp_crm_pipeline_gen::handlers::types::Lead;
use serde_json::{json, Value};
use std::sync::OnceLock;
use std::time::Duration;

// ---------------------------------------------------------------------------
// Stage model — fixed, deterministic definitions for the just-enough CRM.
// The portal receives these from list_stages; ids are stable constants so the
// client can hard-map them and crm_lead_state stores the short code.
// ---------------------------------------------------------------------------

pub struct StageDef {
    pub code: &'static str,
    pub id: &'static str,
    pub name: &'static str,
    pub sequence: i32,
    pub probability: i32,
    pub is_won: bool,
    pub is_lost: bool,
}

pub const STAGES: [StageDef; 5] = [
    StageDef {
        code: "new",
        id: "00000000-0000-0000-0000-000000000101",
        name: "New",
        sequence: 1,
        probability: 10,
        is_won: false,
        is_lost: false,
    },
    StageDef {
        code: "contacted",
        id: "00000000-0000-0000-0000-000000000102",
        name: "Contacted",
        sequence: 2,
        probability: 30,
        is_won: false,
        is_lost: false,
    },
    StageDef {
        code: "invited",
        id: "00000000-0000-0000-0000-000000000103",
        name: "Invited",
        sequence: 3,
        probability: 60,
        is_won: false,
        is_lost: false,
    },
    StageDef {
        code: "converted",
        id: "00000000-0000-0000-0000-000000000104",
        name: "Converted",
        sequence: 4,
        probability: 100,
        is_won: true,
        is_lost: false,
    },
    StageDef {
        code: "lost",
        id: "00000000-0000-0000-0000-000000000105",
        name: "Lost",
        sequence: 5,
        probability: 0,
        is_won: false,
        is_lost: true,
    },
];

/// Deterministic source ids so the portal can distinguish lead origins.
pub const SOURCE_WAITING_LIST: &str = "00000000-0000-0000-0000-000000000201";
pub const SOURCE_CONTACT_FORM: &str = "00000000-0000-0000-0000-000000000202";
/// Tag applied to leads whose email address is verified.
pub const TAG_EMAIL_VERIFIED: &str = "00000000-0000-0000-0000-000000000301";

pub fn stage_by_code(code: &str) -> &'static StageDef {
    STAGES.iter().find(|s| s.code == code).unwrap_or(&STAGES[0])
}

pub fn stage_by_id(id: &str) -> Option<&'static StageDef> {
    STAGES.iter().find(|s| s.id == id)
}

/// Monthly USD price per plan code — mirrors the public pricing page. Used to
/// give leads an honest expected-revenue figure; unknown codes contribute 0.
fn plan_monthly_usd(code: &str) -> f64 {
    match code {
        "trader" => 149.0,
        "professional" => 299.0,
        "desk" => 499.0,
        // Retired tier codes kept for historical rows.
        "starter" => 49.0,
        "growth" => 99.0,
        "pro" => 199.0,
        "enterprise" => 499.0,
        _ => 0.0,
    }
}

// ---------------------------------------------------------------------------
// PostgREST client
// ---------------------------------------------------------------------------

pub struct Supabase {
    client: Client,
    base: String,
    key: String,
}

static SUPABASE: OnceLock<Result<Supabase, String>> = OnceLock::new();

pub fn supabase() -> Result<&'static Supabase, String> {
    SUPABASE
        .get_or_init(Supabase::from_env)
        .as_ref()
        .map_err(Clone::clone)
}

impl Supabase {
    fn from_env() -> Result<Self, String> {
        let base = std::env::var("SUPABASE_URL")
            .map_err(|_| "SUPABASE_URL is not set".to_string())?
            .trim_end_matches('/')
            .to_string();
        if !base.starts_with("https://") {
            return Err("SUPABASE_URL must be https".to_string());
        }
        let key = std::env::var("SUPABASE_SERVICE_ROLE_KEY")
            .map_err(|_| "SUPABASE_SERVICE_ROLE_KEY is not set".to_string())?;
        let client = Client::builder()
            .redirect_policy(RedirectPolicy::None)
            .connect_timeout(Duration::from_secs(5))
            .request_timeout(Duration::from_secs(15))
            .build()
            .map_err(|error| format!("supabase client configuration: {error}"))?;
        Ok(Self { client, base, key })
    }

    fn request(
        &self,
        method: Method,
        path_and_query: &str,
        body: Option<Value>,
    ) -> Result<Value, String> {
        let url = format!("{}{}", self.base, path_and_query);
        let mut req = self
            .client
            .request(method, &url)
            .map_err(|error| format!("supabase request: {error}"))?
            .header_str("apikey", &self.key)
            .map_err(|error| format!("supabase header: {error}"))?
            .header_str("authorization", &format!("Bearer {}", self.key))
            .map_err(|error| format!("supabase header: {error}"))?
            .header_str("accept", "application/json")
            .map_err(|error| format!("supabase header: {error}"))?;
        if let Some(payload) = body {
            req = req
                .header_str("content-type", "application/json")
                .map_err(|error| format!("supabase header: {error}"))?
                .header_str("prefer", "return=representation")
                .map_err(|error| format!("supabase header: {error}"))?
                .body(payload.to_string().into_bytes());
        }
        let response = req
            .send()
            .map_err(|error| format!("supabase request failed: {error}"))?;
        let status = response.status().as_u16();
        if !(200..300).contains(&status) {
            let text = String::from_utf8_lossy(response.body()).into_owned();
            return Err(format!(
                "supabase HTTP {status}: {}",
                text.chars().take(300).collect::<String>()
            ));
        }
        if response.body().is_empty() {
            return Ok(Value::Null);
        }
        serde_json::from_slice(response.body())
            .map_err(|error| format!("supabase response parse: {error}"))
    }

    pub fn get(&self, path_and_query: &str) -> Result<Value, String> {
        self.request(Method::GET, path_and_query, None)
    }

    pub fn post(&self, path_and_query: &str, body: Value) -> Result<Value, String> {
        self.request(Method::POST, path_and_query, Some(body))
    }

    pub fn patch(&self, path_and_query: &str, body: Value) -> Result<Value, String> {
        self.request(Method::PATCH, path_and_query, Some(body))
    }
}

// ---------------------------------------------------------------------------
// Lead assembly
// ---------------------------------------------------------------------------

fn s(v: &Value, key: &str) -> Option<String> {
    v.get(key).and_then(Value::as_str).map(str::to_string)
}

fn nested_s(v: &Value, outer: &str, key: &str) -> Option<String> {
    v.get(outer)
        .and_then(|o| o.get(key))
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn nested_b(v: &Value, outer: &str, key: &str) -> Option<bool> {
    v.get(outer)
        .and_then(|o| o.get(key))
        .and_then(Value::as_bool)
}

/// A lead with every optional field empty. The honest baseline: only what the
/// marketing DB actually knows gets filled in by the assemblers below.
fn empty_lead(id: String, name: String, create_date: String) -> Lead {
    Lead {
        id,
        name,
        r#type: "LEAD".to_string(),
        create_date,
        active: true,
        automated_probability: None,
        campaign_id: None,
        color: None,
        company_id: None,
        company_name: None,
        contact_name: None,
        date_closed: None,
        date_deadline: None,
        date_last_stage_update: None,
        date_open: None,
        day_close: None,
        day_open: None,
        description: None,
        duplicate_lead_count: None,
        duplicate_lead_ids: None,
        email_from: None,
        email_normalized: None,
        expected_revenue: None,
        function: None,
        is_automated_probability: None,
        is_blacklisted: None,
        is_rotting: None,
        lost_reason_id: None,
        medium_id: None,
        mobile: None,
        partner_id: None,
        phone: None,
        phone_sanitized: None,
        priority: None,
        probability: None,
        prorated_revenue: None,
        recurring_plan_id: None,
        recurring_revenue: None,
        recurring_revenue_monthly: None,
        referred_by: None,
        source_id: None,
        stage_color: None,
        stage_id: None,
        stage_name: None,
        stage_probability: None,
        tag_ids: None,
        team_id: None,
        title: None,
        user_id: None,
        website: None,
        won_status: None,
        write_date: None,
        write_uid: None,
    }
}

fn apply_state(lead: &mut Lead, state: Option<&Value>) {
    let stage = state
        .and_then(|st| s(st, "stage"))
        .unwrap_or_else(|| "new".to_string());
    let def = stage_by_code(&stage);
    lead.stage_id = Some(def.id.to_string());
    lead.stage_name = Some(def.name.to_string());
    lead.stage_probability = Some(def.probability);
    lead.probability = Some(def.probability as f64);
    lead.won_status = Some(
        if def.is_won {
            "WON"
        } else if def.is_lost {
            "LOST"
        } else {
            "PENDING"
        }
        .to_string(),
    );
    if let Some(st) = state {
        // The triage note overrides the initial description (original message /
        // signup source line) once someone has written one.
        if let Some(note) = s(st, "note") {
            if !note.is_empty() {
                lead.description = Some(note);
            }
        }
        lead.write_date = s(st, "updated_at");
        lead.date_last_stage_update = s(st, "updated_at");
    }
}

fn capture_to_lead(row: &Value, state: Option<&Value>) -> Lead {
    let id = s(row, "id").unwrap_or_default();
    let email = nested_s(row, "email_addresses", "email").unwrap_or_default();
    let name = s(row, "name")
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| email.clone());
    let created = s(row, "created_at").unwrap_or_default();
    let mut lead = empty_lead(id, name, created);
    lead.contact_name = s(row, "name");
    lead.email_from = Some(email.clone());
    lead.email_normalized = Some(email.to_lowercase());
    lead.company_id = s(row, "company_id");
    lead.company_name = nested_s(row, "companies", "name");
    lead.source_id = Some(SOURCE_WAITING_LIST.to_string());
    // Which form placement captured them: hero / exit_intent / free_trial.
    lead.referred_by = s(row, "source");
    if nested_b(row, "email_addresses", "verified").unwrap_or(false) {
        lead.tag_ids = Some(vec![TAG_EMAIL_VERIFIED.to_string()]);
    }
    if let Some(plan) = row.get("plans").filter(|p| !p.is_null()) {
        let code = plan.get("code").and_then(Value::as_str).unwrap_or("");
        let monthly = plan_monthly_usd(code);
        lead.recurring_plan_id = s(row, "plan_id");
        if monthly > 0.0 {
            lead.recurring_revenue = Some(monthly);
            lead.recurring_revenue_monthly = Some(monthly);
            lead.expected_revenue = Some(monthly * 12.0);
        }
        lead.description = Some(format!(
            "Waiting-list signup — interested in {}",
            plan.get("name").and_then(Value::as_str).unwrap_or(code)
        ));
    } else {
        lead.description = Some("Waiting-list signup".to_string());
    }
    apply_state(&mut lead, state);
    lead
}

fn message_to_lead(row: &Value, state: Option<&Value>) -> Lead {
    let id = s(row, "id").unwrap_or_default();
    let email = nested_s(row, "email_addresses", "email").unwrap_or_default();
    let name = s(row, "name")
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| email.clone());
    let created = s(row, "created_at").unwrap_or_default();
    let mut lead = empty_lead(id, name, created);
    lead.contact_name = s(row, "name");
    lead.email_from = Some(email.clone());
    lead.email_normalized = Some(email.to_lowercase());
    lead.company_id = s(row, "company_id");
    lead.company_name = nested_s(row, "companies", "name");
    lead.source_id = Some(SOURCE_CONTACT_FORM.to_string());
    lead.referred_by = Some("contact_form".to_string());
    if nested_b(row, "email_addresses", "verified").unwrap_or(false) {
        lead.tag_ids = Some(vec![TAG_EMAIL_VERIFIED.to_string()]);
    }
    lead.description = s(row, "message");
    apply_state(&mut lead, state);
    lead
}

/// Fetch every lead (both sources), newest first. Volumes are private-beta
/// sized; when signups outgrow one page this becomes a proper pushdown query.
pub fn fetch_leads() -> Result<Vec<Lead>, String> {
    let sb = supabase()?;
    let captures = sb.get(
        "/rest/v1/email_captures?select=id,name,source,created_at,company_id,plan_id,email_addresses(email,verified),companies(name),plans(code,name)&order=created_at.desc&limit=1000",
    )?;
    let messages = sb.get(
        "/rest/v1/contact_messages?select=id,name,message,created_at,company_id,email_addresses(email,verified),companies(name)&order=created_at.desc&limit=1000",
    )?;
    let states =
        sb.get("/rest/v1/crm_lead_state?select=lead_id,stage,note,updated_at&limit=10000")?;

    let empty = Vec::new();
    let states = states.as_array().unwrap_or(&empty);
    let state_for = |id: &str| -> Option<&Value> {
        states
            .iter()
            .find(|st| st.get("lead_id").and_then(Value::as_str) == Some(id))
    };

    let mut leads: Vec<Lead> = Vec::new();
    for row in captures.as_array().unwrap_or(&empty) {
        let id = s(row, "id").unwrap_or_default();
        leads.push(capture_to_lead(row, state_for(&id)));
    }
    for row in messages.as_array().unwrap_or(&empty) {
        let id = s(row, "id").unwrap_or_default();
        leads.push(message_to_lead(row, state_for(&id)));
    }
    leads.sort_by(|a, b| b.create_date.cmp(&a.create_date));
    Ok(leads)
}

pub fn fetch_lead(id: &str) -> Result<Option<Lead>, String> {
    // Two point lookups beat scanning both tables; the id lives in exactly one.
    let sb = supabase()?;
    let enc: String = id
        .chars()
        .filter(|c| c.is_ascii_hexdigit() || *c == '-')
        .collect();
    if enc.len() != 36 {
        return Ok(None);
    }
    let state = sb.get(&format!(
        "/rest/v1/crm_lead_state?select=lead_id,stage,note,updated_at&lead_id=eq.{enc}"
    ))?;
    let state_row = state.as_array().and_then(|a| a.first()).cloned();
    let captures = sb.get(&format!(
        "/rest/v1/email_captures?select=id,name,source,created_at,company_id,plan_id,email_addresses(email,verified),companies(name),plans(code,name)&id=eq.{enc}"
    ))?;
    if let Some(row) = captures.as_array().and_then(|a| a.first()) {
        return Ok(Some(capture_to_lead(row, state_row.as_ref())));
    }
    let messages = sb.get(&format!(
        "/rest/v1/contact_messages?select=id,name,message,created_at,company_id,email_addresses(email,verified),companies(name)&id=eq.{enc}"
    ))?;
    if let Some(row) = messages.as_array().and_then(|a| a.first()) {
        return Ok(Some(message_to_lead(row, state_row.as_ref())));
    }
    Ok(None)
}

/// Write triage state. `stage` and `note` update only what is provided.
pub fn write_state(lead_id: &str, stage: Option<&str>, note: Option<&str>) -> Result<(), String> {
    let sb = supabase()?;
    let enc: String = lead_id
        .chars()
        .filter(|c| c.is_ascii_hexdigit() || *c == '-')
        .collect();
    if enc.len() != 36 {
        return Err("invalid lead id".to_string());
    }
    let existing = sb.get(&format!(
        "/rest/v1/crm_lead_state?select=lead_id&lead_id=eq.{enc}"
    ))?;
    let exists = existing.as_array().map(|a| !a.is_empty()).unwrap_or(false);
    let mut body = json!({ "updated_at": chrono::Utc::now().to_rfc3339() });
    if let Some(stg) = stage {
        body["stage"] = json!(stage_by_code(stg).code);
    }
    if let Some(n) = note {
        body["note"] = json!(n);
    }
    if exists {
        sb.patch(&format!("/rest/v1/crm_lead_state?lead_id=eq.{enc}"), body)?;
    } else {
        body["lead_id"] = json!(enc);
        if body.get("stage").is_none() {
            body["stage"] = json!("new");
        }
        sb.post("/rest/v1/crm_lead_state", body)?;
    }
    Ok(())
}
