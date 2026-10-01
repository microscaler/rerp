//! Supabase marketing-DB access over PostgREST.
//!
//! The just-enough CRM reads the PriceWhisperer marketing database (Supabase
//! project `marketing`) directly over its REST API. Leads are a live view of
//! three website tables — `email_captures` (waiting-list signups),
//! `contact_messages` (contact form) and `job_applications` (careers) —
//! joined with `email_addresses`,
//! `companies` and `plans`. Triage state (stage, priority, note) is the one
//! thing the CRM owns: the `crm_lead_state` table, keyed by the source row's
//! UUID. Each lead also carries what the submitter entered (`form_fields`) and
//! their first-touch traffic source (`attribution`), both read-only.
//!
//! All HTTP goes through the coroutine-native `may_minihttp` client (rustls);
//! tokio-based clients cannot run inside a `may` service. The service_role
//! key comes from the environment (k8s Secret) and never reaches a browser.

use http_legacy::Method;
use may_minihttp::client::{Client, RedirectPolicy};
use rerp_crm_pipeline_gen::handlers::types::{Lead, LeadAttribution, LeadFormField};
use serde_json::{json, Value};
use std::sync::OnceLock;
use std::time::Duration;


// ---------------------------------------------------------------------------
// Pipelines and stages — fixed, deterministic definitions (Odoo-style: a
// stage belongs to a team, and each team is a pipeline on the Kanban).
// Ids are stable constants so the portal can hard-map them; crm_lead_state
// stores the short code (its CHECK lists every code, migration 29).
// ---------------------------------------------------------------------------

/// Sales pipeline: waiting list, launch waitlist and contact-form leads.
pub const TEAM_SALES: &str = "00000000-0000-0000-0000-000000000401";
/// Recruitment pipeline: careers applications.
pub const TEAM_RECRUITMENT: &str = "00000000-0000-0000-0000-000000000402";

pub struct StageDef {
    pub code: &'static str,
    pub id: &'static str,
    pub name: &'static str,
    pub team: &'static str,
    pub sequence: i32,
    pub probability: i32,
    pub is_won: bool,
    pub is_lost: bool,
    /// Folded (collapsed) on the Kanban: the closed-out lane.
    pub fold: bool,
    /// Odoo colour index (1-16) for the column header.
    pub color: i32,
    pub requirements: &'static str,
}

const fn stage(
    code: &'static str,
    id: &'static str,
    name: &'static str,
    team: &'static str,
    sequence: i32,
    probability: i32,
    is_won: bool,
    is_lost: bool,
    color: i32,
    requirements: &'static str,
) -> StageDef {
    StageDef { code, id, name, team, sequence, probability, is_won, is_lost, fold: is_lost, color, requirements }
}

pub const STAGES: [StageDef; 11] = [
    stage("new", "00000000-0000-0000-0000-000000000101", "New", TEAM_SALES, 1, 10, false, false, 4,
        "Signed up or wrote in; nobody has replied yet"),
    stage("contacted", "00000000-0000-0000-0000-000000000102", "Contacted", TEAM_SALES, 2, 30, false, false, 3,
        "We replied; waiting on them"),
    stage("invited", "00000000-0000-0000-0000-000000000103", "Invited", TEAM_SALES, 3, 60, false, false, 7,
        "Sent an invitation to the beta"),
    stage("converted", "00000000-0000-0000-0000-000000000104", "Converted", TEAM_SALES, 4, 100, true, false, 10,
        "Signed in and using the product"),
    stage("lost", "00000000-0000-0000-0000-000000000105", "Lost", TEAM_SALES, 5, 0, false, true, 1,
        "Not a fit, unreachable or a test row"),
    stage("applied", "00000000-0000-0000-0000-000000000111", "Applied", TEAM_RECRUITMENT, 1, 10, false, false, 4,
        "Application received; not yet reviewed"),
    stage("screening", "00000000-0000-0000-0000-000000000112", "Screening", TEAM_RECRUITMENT, 2, 25, false, false, 3,
        "CV and experience under review"),
    stage("interview", "00000000-0000-0000-0000-000000000113", "Interview", TEAM_RECRUITMENT, 3, 50, false, false, 7,
        "Interviews scheduled or in progress"),
    stage("offer", "00000000-0000-0000-0000-000000000114", "Offer", TEAM_RECRUITMENT, 4, 80, false, false, 2,
        "Offer made; waiting for an answer"),
    stage("hired", "00000000-0000-0000-0000-000000000115", "Hired", TEAM_RECRUITMENT, 5, 100, true, false, 10,
        "Accepted"),
    stage("refused", "00000000-0000-0000-0000-000000000116", "Refused", TEAM_RECRUITMENT, 6, 0, false, true, 1,
        "Declined, withdrawn or not a fit"),
];

/// Deterministic source ids so the portal can distinguish lead origins.
pub const SOURCE_WAITING_LIST: &str = "00000000-0000-0000-0000-000000000201";
pub const SOURCE_CONTACT_FORM: &str = "00000000-0000-0000-0000-000000000202";
pub const SOURCE_CAREERS: &str = "00000000-0000-0000-0000-000000000203";

/// First-touch attribution columns every marketing form records (migrations 25, 29).
const ATTRIBUTION_COLS: &str = "utm_source,utm_medium,utm_campaign,utm_content,utm_term,link_code,referrer,landing_path,first_touch_at,affiliate_ref";

fn capture_select() -> String {
    format!("id,name,source,created_at,company_id,plan_id,{ATTRIBUTION_COLS},email_addresses(email,verified),companies(name),plans(code,name)")
}
fn message_select() -> String {
    format!("id,name,message,created_at,company_id,{ATTRIBUTION_COLS},email_addresses(email,verified),companies(name)")
}
fn application_select() -> String {
    format!("id,first_name,last_name,phone,job_short_id,job_title,location,linkedin,github,technologies,other_technologies,work_experience,interest,created_at,{ATTRIBUTION_COLS},email_addresses(email,verified)")
}
const STATE_SELECT: &str = "lead_id,stage,note,priority,updated_at";

/// Tag applied to leads whose email address is verified.
pub const TAG_EMAIL_VERIFIED: &str = "00000000-0000-0000-0000-000000000301";

/// The stage for `code` within `team`'s pipeline; an unknown or foreign code
/// (a careers row still carrying a sales code) lands in that pipeline's first.
pub fn stage_in_team(code: &str, team: &str) -> &'static StageDef {
    STAGES
        .iter()
        .find(|s| s.code == code && s.team == team)
        .or_else(|| STAGES.iter().filter(|s| s.team == team).min_by_key(|s| s.sequence))
        .unwrap_or(&STAGES[0])
}

pub fn stage_by_id(id: &str) -> Option<&'static StageDef> {
    STAGES.iter().find(|s| s.id == id)
}

pub fn stages_for(team: Option<&str>) -> impl Iterator<Item = &'static StageDef> + '_ {
    STAGES.iter().filter(move |s| team.map_or(true, |t| s.team == t))
}

/// Odoo priority (0-3 stars) <-> the spec's priority enum.
pub fn priority_name(stars: i64) -> &'static str {
    match stars {
        i64::MIN..=0 => "LOW",
        1 => "NORMAL",
        2 => "HIGH",
        _ => "URGENT",
    }
}
pub fn priority_stars(name: &str) -> Option<i16> {
    match name {
        "LOW" => Some(0),
        "NORMAL" => Some(1),
        "HIGH" => Some(2),
        "URGENT" => Some(3),
        _ => None,
    }
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


/// Make a lead fit the `Lead` response schema whatever the marketing forms
/// stored. The router validates every response; one bad row (an empty email,
/// a "linkedin.com/in/x" without a scheme, an over-long field) used to fail
/// the WHOLE list with "Response validation failed". Values that cannot be
/// made valid are dropped, never invented; over-long text is truncated.
fn conform(mut lead: Lead) -> Lead {
    fn cut(v: String, max: usize) -> String {
        if v.chars().count() <= max { v } else { v.chars().take(max).collect() }
    }
    fn cut_opt(v: Option<String>, max: usize) -> Option<String> {
        v.map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).map(|x| cut(x, max))
    }
    fn email_ok(e: &str) -> bool {
        let e = e.trim();
        let Some((local, domain)) = e.split_once('@') else { return false };
        !local.is_empty()
            && domain.contains('.')
            && !domain.starts_with('.')
            && !domain.ends_with('.')
            && !e.chars().any(char::is_whitespace)
            && e.len() <= 255
    }
    fn uri(w: String) -> Option<String> {
        let w = w.trim().to_string();
        if w.is_empty() || w.chars().any(char::is_whitespace) {
            return None;
        }
        let w = if w.starts_with("http://") || w.starts_with("https://") {
            w
        } else if w.contains('.') && !w.contains("://") {
            format!("https://{w}")
        } else {
            return None;
        };
        (w.len() <= 255).then_some(w)
    }
    lead.name = cut(lead.name.trim().to_string(), 255);
    if lead.name.is_empty() {
        lead.name = "(no name)".to_string();
    }
    lead.email_from = lead.email_from.filter(|e| email_ok(e)).map(|e| e.trim().to_string());
    lead.email_normalized = cut_opt(lead.email_normalized, 255);
    lead.contact_name = cut_opt(lead.contact_name, 255);
    lead.company_name = cut_opt(lead.company_name, 255);
    lead.referred_by = cut_opt(lead.referred_by, 255);
    lead.function = cut_opt(lead.function, 128);
    lead.phone = cut_opt(lead.phone, 64);
    lead.phone_sanitized = cut_opt(lead.phone_sanitized, 64);
    lead.mobile = cut_opt(lead.mobile, 64);
    lead.website = lead.website.and_then(uri);
    lead.title = lead.title.filter(|t| matches!(t.as_str(), "MR" | "MRS" | "MME"));
    lead
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
        ..Default::default()
    }
}

/// Stage, priority and triage note from crm_lead_state, within `team`'s
/// pipeline. The note is the CRM's own text; what the submitter wrote is in
/// `form_fields` and is never overwritten.
fn apply_state(lead: &mut Lead, state: Option<&Value>, team: &'static str) {
    let code = state.and_then(|st| s(st, "stage")).unwrap_or_default();
    let def = stage_in_team(&code, team);
    lead.team_id = Some(team.to_string());
    lead.stage_id = Some(def.id.to_string());
    lead.stage_name = Some(def.name.to_string());
    lead.stage_probability = Some(def.probability);
    lead.stage_color = Some(def.color);
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
    let stars = state.and_then(|st| st.get("priority")).and_then(Value::as_i64).unwrap_or(0);
    lead.priority = Some(priority_name(stars).to_string());
    if let Some(st) = state {
        lead.description = s(st, "note").filter(|n| !n.trim().is_empty());
        lead.write_date = s(st, "updated_at");
        lead.date_last_stage_update = s(st, "updated_at");
    }
}

/// The submitter's first-touch traffic source, or None when the row predates
/// attribution (no column set). `channel` is the one-line summary the board
/// shows on cards: an affiliate first, then the campaign, then the referrer.
fn attribution_of(row: &Value) -> Option<LeadAttribution> {
    let get = |k: &str| s(row, k).map(|v| v.trim().to_string()).filter(|v| !v.is_empty());
    let mut a = LeadAttribution {
        utm_source: get("utm_source"),
        utm_medium: get("utm_medium"),
        utm_campaign: get("utm_campaign"),
        utm_content: get("utm_content"),
        utm_term: get("utm_term"),
        link_code: get("link_code"),
        referrer: get("referrer"),
        landing_path: get("landing_path"),
        first_touch_at: get("first_touch_at"),
        affiliate_ref: get("affiliate_ref"),
        channel: None,
    };
    let any = [
        &a.utm_source, &a.utm_medium, &a.utm_campaign, &a.utm_content, &a.utm_term,
        &a.link_code, &a.referrer, &a.landing_path, &a.first_touch_at, &a.affiliate_ref,
    ]
    .iter()
    .any(|v| v.is_some());
    if !any {
        return None;
    }
    let referrer_host = a.referrer.as_deref().and_then(|r| {
        let rest = r.split("://").nth(1).unwrap_or(r);
        rest.split(['/', '?', '#']).next().map(str::to_string).filter(|h| !h.is_empty())
    });
    let campaign = match (a.utm_source.as_deref(), a.utm_medium.as_deref()) {
        (Some("direct"), _) | (None, Some("none")) => None,
        (Some(src), Some(med)) => Some(format!("{src} / {med}")),
        (Some(src), None) => Some(src.to_string()),
        _ => None,
    };
    let base = campaign
        .or_else(|| referrer_host.map(|h| format!("{h} (referral)")))
        .unwrap_or_else(|| "direct".to_string());
    a.channel = Some(match a.affiliate_ref.as_deref() {
        Some(r) => format!("FirstPromoter: {r} · {base}"),
        None => base,
    });
    Some(a)
}

fn field(key: &str, label: &str, kind: &str, value: Option<String>) -> Option<LeadFormField> {
    let value = value.map(|v| v.trim().to_string()).filter(|v| !v.is_empty())?;
    Some(LeadFormField {
        key: key.to_string(),
        label: label.to_string(),
        kind: kind.to_string(),
        value: Some(value),
        values: None,
        entries: None,
    })
}

fn form_label(form: &str) -> &'static str {
    match form {
        "hero" => "Homepage hero",
        "exit_intent" => "Exit-intent popup",
        "free_trial" => "Waiting-list page",
        "launch_waitlist" => "Launch waitlist page",
        "contact" => "Contact form",
        "careers" => "Careers application",
        _ => "Website form",
    }
}

fn email_fields(row: &Value) -> Vec<Option<LeadFormField>> {
    let verified = nested_b(row, "email_addresses", "verified").unwrap_or(false);
    vec![
        field("email", "Email", "email", nested_s(row, "email_addresses", "email")),
        field("email_verified", "Email verified", "text", Some(if verified { "Yes" } else { "No" }.to_string())),
    ]
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
    let form = s(row, "source").unwrap_or_else(|| "free_trial".to_string());
    // Which form placement captured them: hero / exit_intent / free_trial / launch_waitlist.
    lead.referred_by = Some(form.clone());
    if nested_b(row, "email_addresses", "verified").unwrap_or(false) {
        lead.tag_ids = Some(vec![TAG_EMAIL_VERIFIED.to_string()]);
    }
    let mut plan_text = None;
    if let Some(plan) = row.get("plans").filter(|p| !p.is_null()) {
        let code = plan.get("code").and_then(Value::as_str).unwrap_or("");
        let pname = plan.get("name").and_then(Value::as_str).unwrap_or(code);
        let monthly = plan_monthly_usd(code);
        lead.recurring_plan_id = s(row, "plan_id");
        if monthly > 0.0 {
            lead.recurring_revenue = Some(monthly);
            lead.recurring_revenue_monthly = Some(monthly);
            lead.expected_revenue = Some(monthly * 12.0);
            plan_text = Some(format!("{pname} (${monthly:.0}/mo)"));
        } else if !pname.is_empty() {
            plan_text = Some(pname.to_string());
        }
    }
    let mut fields = vec![
        field("name", "Name", "text", s(row, "name")),
    ];
    fields.extend(email_fields(row));
    fields.extend([
        field("company", "Company", "text", nested_s(row, "companies", "name")),
        field("plan", "Plan interest", "text", plan_text),
        field("form", "Signed up on", "text", Some(form_label(&form).to_string())),
    ]);
    lead.form = Some(form);
    lead.form_fields = Some(fields.into_iter().flatten().collect());
    lead.attribution = attribution_of(row);
    apply_state(&mut lead, state, TEAM_SALES);
    conform(lead)
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
    let mut fields = vec![field("name", "Name", "text", s(row, "name"))];
    fields.extend(email_fields(row));
    fields.extend([
        field("company", "Company", "text", nested_s(row, "companies", "name")),
        field("message", "Message", "longtext", s(row, "message")),
    ]);
    lead.form = Some("contact".to_string());
    lead.form_fields = Some(fields.into_iter().flatten().collect());
    lead.attribution = attribution_of(row);
    apply_state(&mut lead, state, TEAM_SALES);
    conform(lead)
}

fn application_to_lead(row: &Value, state: Option<&Value>) -> Lead {
    let id = s(row, "id").unwrap_or_default();
    let email = nested_s(row, "email_addresses", "email").unwrap_or_default();
    let who = format!(
        "{} {}",
        s(row, "first_name").unwrap_or_default(),
        s(row, "last_name").unwrap_or_default()
    )
    .trim()
    .to_string();
    let name = if who.is_empty() { email.clone() } else { who.clone() };
    let created = s(row, "created_at").unwrap_or_default();
    let mut lead = empty_lead(id, name, created);
    lead.contact_name = Some(who.clone());
    lead.email_from = Some(email.clone());
    lead.email_normalized = Some(email.to_lowercase());
    lead.phone = s(row, "phone");
    lead.source_id = Some(SOURCE_CAREERS.to_string());
    // The role applied for (job short id), like referred_by carries the form placement.
    lead.referred_by = s(row, "job_short_id").or_else(|| Some("careers".to_string()));
    // The role goes in `function` (job position, max 128). `title` is the
    // salutation enum (MR/MRS/MME): putting the job title there failed the
    // response schema and took the whole leads list down with a 500.
    lead.function = s(row, "job_title");
    lead.website = s(row, "linkedin")
        .filter(|w| !w.trim().is_empty())
        .or_else(|| s(row, "github"));
    if nested_b(row, "email_addresses", "verified").unwrap_or(false) {
        lead.tag_ids = Some(vec![TAG_EMAIL_VERIFIED.to_string()]);
    }
    let role = match (s(row, "job_title"), s(row, "job_short_id")) {
        (Some(t), Some(j)) => Some(format!("{t} ({j})")),
        (Some(t), None) => Some(t),
        (None, Some(j)) => Some(j),
        (None, None) => Some("General application".to_string()),
    };
    let technologies: Vec<String> = row
        .get("technologies")
        .and_then(Value::as_array)
        .map(|t| t.iter().filter_map(Value::as_str).map(str::to_string).collect())
        .unwrap_or_default();
    // Work history exactly as entered: company, jobTitle, startDate, endDate,
    // background, technicalExperience. Shown as cards; never reshaped.
    let experience: Vec<Value> = row
        .get("work_experience")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter(|e| {
                    e.as_object()
                        .map(|o| o.values().any(|v| v.as_str().map_or(!v.is_null(), |s| !s.trim().is_empty())))
                        .unwrap_or(false)
                })
                .cloned()
                .collect()
        })
        .unwrap_or_default();
    let mut fields = vec![
        field("name", "Name", "text", Some(who)),
    ];
    fields.extend(email_fields(row));
    fields.extend([
        field("phone", "Phone", "phone", s(row, "phone")),
        field("location", "Location", "text", s(row, "location")),
        field("role", "Role applied for", "text", role),
        field("linkedin", "LinkedIn", "url", s(row, "linkedin")),
        field("github", "GitHub", "url", s(row, "github")),
    ]);
    if !technologies.is_empty() {
        fields.push(Some(LeadFormField {
            key: "technologies".into(),
            label: "Technologies".into(),
            kind: "list".into(),
            value: None,
            values: Some(technologies),
            entries: None,
        }));
    }
    fields.push(field("other_technologies", "Other technologies", "text", s(row, "other_technologies")));
    if !experience.is_empty() {
        fields.push(Some(LeadFormField {
            key: "work_experience".into(),
            label: "Work experience".into(),
            kind: "experience".into(),
            value: None,
            values: None,
            entries: Some(experience),
        }));
    }
    fields.push(field("interest", "Why PriceWhisperer", "longtext", s(row, "interest")));
    lead.form = Some("careers".to_string());
    lead.form_fields = Some(fields.into_iter().flatten().collect());
    lead.attribution = attribution_of(row);
    apply_state(&mut lead, state, TEAM_RECRUITMENT);
    conform(lead)
}

/// Fetch every lead (all three forms), newest first. Volumes are private-beta
/// sized; when signups outgrow one page this becomes a proper pushdown query.
pub fn fetch_leads() -> Result<Vec<Lead>, String> {
    let sb = supabase()?;
    let captures = sb.get(&format!(
        "/rest/v1/email_captures?select={}&order=created_at.desc&limit=1000",
        capture_select()
    ))?;
    let messages = sb.get(&format!(
        "/rest/v1/contact_messages?select={}&order=created_at.desc&limit=1000",
        message_select()
    ))?;
    let applications = sb.get(&format!(
        "/rest/v1/job_applications?select={}&order=created_at.desc&limit=1000",
        application_select()
    ))?;
    let states = sb.get(&format!("/rest/v1/crm_lead_state?select={STATE_SELECT}&limit=10000"))?;

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
    for row in applications.as_array().unwrap_or(&empty) {
        let id = s(row, "id").unwrap_or_default();
        leads.push(application_to_lead(row, state_for(&id)));
    }
    leads.sort_by(|a, b| b.create_date.cmp(&a.create_date));
    Ok(leads)
}

fn clean_id(id: &str) -> Option<String> {
    let enc: String = id
        .chars()
        .filter(|c| c.is_ascii_hexdigit() || *c == '-')
        .collect();
    (enc.len() == 36).then_some(enc)
}

pub fn fetch_lead(id: &str) -> Result<Option<Lead>, String> {
    // Point lookups beat scanning every table; the id lives in exactly one.
    let sb = supabase()?;
    let Some(enc) = clean_id(id) else { return Ok(None) };
    let state = sb.get(&format!(
        "/rest/v1/crm_lead_state?select={STATE_SELECT}&lead_id=eq.{enc}"
    ))?;
    let state_row = state.as_array().and_then(|a| a.first()).cloned();
    let captures = sb.get(&format!(
        "/rest/v1/email_captures?select={}&id=eq.{enc}",
        capture_select()
    ))?;
    if let Some(row) = captures.as_array().and_then(|a| a.first()) {
        return Ok(Some(capture_to_lead(row, state_row.as_ref())));
    }
    let messages = sb.get(&format!(
        "/rest/v1/contact_messages?select={}&id=eq.{enc}",
        message_select()
    ))?;
    if let Some(row) = messages.as_array().and_then(|a| a.first()) {
        return Ok(Some(message_to_lead(row, state_row.as_ref())));
    }
    let applications = sb.get(&format!(
        "/rest/v1/job_applications?select={}&id=eq.{enc}",
        application_select()
    ))?;
    if let Some(row) = applications.as_array().and_then(|a| a.first()) {
        return Ok(Some(application_to_lead(row, state_row.as_ref())));
    }
    Ok(None)
}

/// A triage change: only what is `Some` is written.
#[derive(Default)]
pub struct StateChange<'a> {
    /// Stage code, already validated against the lead's pipeline.
    pub stage: Option<&'static str>,
    pub note: Option<&'a str>,
    pub priority: Option<i16>,
}

/// Write triage state for `lead`. A first write for a lead also records the
/// lead's current stage, so a note or a star never resets the column.
pub fn write_state(lead: &Lead, change: StateChange<'_>) -> Result<(), String> {
    let sb = supabase()?;
    let enc = clean_id(&lead.id).ok_or_else(|| "invalid lead id".to_string())?;
    let existing = sb.get(&format!(
        "/rest/v1/crm_lead_state?select=lead_id&lead_id=eq.{enc}"
    ))?;
    let exists = existing.as_array().map(|a| !a.is_empty()).unwrap_or(false);
    let mut body = json!({ "updated_at": chrono::Utc::now().to_rfc3339() });
    if let Some(code) = change.stage {
        body["stage"] = json!(code);
    }
    if let Some(n) = change.note {
        body["note"] = json!(n);
    }
    if let Some(p) = change.priority {
        body["priority"] = json!(p.clamp(0, 3));
    }
    if exists {
        sb.patch(&format!("/rest/v1/crm_lead_state?lead_id=eq.{enc}"), body)?;
    } else {
        body["lead_id"] = json!(enc);
        if body.get("stage").is_none() {
            let current = lead
                .stage_id
                .as_deref()
                .and_then(stage_by_id)
                .map(|d| d.code)
                .unwrap_or("new");
            body["stage"] = json!(current);
        }
        sb.post("/rest/v1/crm_lead_state", body)?;
    }
    Ok(())
}

/// The stage `stage_id` names, if it belongs to `lead`'s pipeline. Moving a
/// careers applicant into a sales column (or back) is refused, not guessed.
pub fn stage_for_lead(lead: &Lead, stage_id: &str) -> Result<&'static StageDef, String> {
    let def = stage_by_id(stage_id).ok_or_else(|| "unknown stage_id".to_string())?;
    match lead.team_id.as_deref() {
        Some(team) if team != def.team => Err(format!(
            "stage {} belongs to the other pipeline",
            def.name
        )),
        _ => Ok(def),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn charles_row() -> Value {
        // The row that took /leads down on 1 Oct 2026, with its work history.
        json!({
            "id": "06f8b0e2-98e5-4c65-ac07-1e31131787a4",
            "first_name": "Ada", "last_name": "Lovelace", "phone": "+359 888 000 000",
            "job_short_id": "6x2y4z8ab",
            "job_title": "Senior Backend Engineer - Exchange Integration",
            "location": "Bansko, Bulgaria",
            "linkedin": "linkedin.com/in/ada", "github": null,
            "technologies": ["Rust", "FluxCD"],
            "work_experience": [
                {"company": "Analytical Engines", "jobTitle": "Engineer", "startDate": "1842", "endDate": "1843",
                 "background": "Notes", "technicalExperience": "Bernoulli numbers"},
                {"company": "", "jobTitle": "", "startDate": "", "endDate": "", "background": "", "technicalExperience": ""}
            ],
            "interest": "I want to work on the forefront of technology",
            "utm_source": "x", "utm_campaign": "launch_waitlist", "utm_medium": "social",
            "created_at": "2026-09-30T10:00:00+00:00",
            "email_addresses": { "email": "ada@example.com", "verified": false }
        })
    }

    #[test]
    fn careers_is_a_recruitment_lead_with_every_field() {
        let lead = application_to_lead(&charles_row(), None);
        assert_eq!(lead.title, None, "title is the salutation enum");
        assert_eq!(lead.function.as_deref(), Some("Senior Backend Engineer - Exchange Integration"));
        assert_eq!(lead.website.as_deref(), Some("https://linkedin.com/in/ada"));
        assert_eq!(lead.team_id.as_deref(), Some(TEAM_RECRUITMENT));
        assert_eq!(lead.stage_name.as_deref(), Some("Applied"));
        assert_eq!(lead.form.as_deref(), Some("careers"));
        assert_eq!(lead.description, None, "no triage note yet; the submission lives in form_fields");
        let f = lead.form_fields.unwrap();
        let keys: Vec<&str> = f.iter().map(|x| x.key.as_str()).collect();
        for k in ["name", "email", "phone", "location", "role", "linkedin", "technologies", "work_experience", "interest"] {
            assert!(keys.contains(&k), "{k} missing from {keys:?}");
        }
        let exp = f.iter().find(|x| x.key == "work_experience").unwrap();
        assert_eq!(exp.entries.as_ref().unwrap().len(), 1, "the empty trailing role is dropped");
        let role = f.iter().find(|x| x.key == "role").unwrap();
        assert_eq!(role.value.as_deref(), Some("Senior Backend Engineer - Exchange Integration (6x2y4z8ab)"));
        assert_eq!(lead.attribution.unwrap().channel.as_deref(), Some("x / social"));
    }

    #[test]
    fn sales_codes_on_a_careers_row_fall_back_to_the_recruitment_pipeline() {
        let st = json!({"lead_id": "06f8b0e2-98e5-4c65-ac07-1e31131787a4", "stage": "invited", "priority": 2, "note": "call Tues"});
        let lead = application_to_lead(&charles_row(), Some(&st));
        assert_eq!(lead.stage_name.as_deref(), Some("Applied"));
        assert_eq!(lead.priority.as_deref(), Some("HIGH"));
        assert_eq!(lead.description.as_deref(), Some("call Tues"));
        let interview = STAGES.iter().find(|s| s.code == "interview").unwrap();
        assert!(stage_for_lead(&lead, interview.id).is_ok());
        let invited = STAGES.iter().find(|s| s.code == "invited").unwrap();
        assert!(stage_for_lead(&lead, invited.id).is_err(), "no sales columns for an applicant");
    }

    #[test]
    fn attribution_channel_prefers_affiliate_then_campaign_then_referrer() {
        let none = json!({"id": "x"});
        assert!(attribution_of(&none).is_none(), "rows from before attribution show nothing");
        let aff = json!({"affiliate_ref": "bob", "utm_source": "youtube", "utm_medium": "influencer"});
        assert_eq!(attribution_of(&aff).unwrap().channel.as_deref(), Some("FirstPromoter: bob · youtube / influencer"));
        let referral = json!({"utm_source": "news.ycombinator.com", "utm_medium": "referral", "referrer": "https://news.ycombinator.com/item?id=1"});
        assert_eq!(attribution_of(&referral).unwrap().channel.as_deref(), Some("news.ycombinator.com / referral"));
        let bare = json!({"referrer": "https://example.org/post"});
        assert_eq!(attribution_of(&bare).unwrap().channel.as_deref(), Some("example.org (referral)"));
        let direct = json!({"utm_source": "direct", "utm_medium": "none", "landing_path": "/launch-waitlist"});
        assert_eq!(attribution_of(&direct).unwrap().channel.as_deref(), Some("direct"));
    }

    #[test]
    fn launch_waitlist_is_its_own_form() {
        let row = json!({"id": "6bc849f1-1e20-4cd3-9a75-fc90964e911d", "name": "Grace", "source": "launch_waitlist",
            "created_at": "2026-10-01T10:00:00+00:00", "email_addresses": {"email": "g@example.com", "verified": true},
            "plans": {"code": "professional", "name": "Professional"}});
        let lead = capture_to_lead(&row, None);
        assert_eq!(lead.form.as_deref(), Some("launch_waitlist"));
        assert_eq!(lead.team_id.as_deref(), Some(TEAM_SALES));
        let f = lead.form_fields.unwrap();
        assert!(f.iter().any(|x| x.key == "form" && x.value.as_deref() == Some("Launch waitlist page")));
        assert!(f.iter().any(|x| x.key == "plan" && x.value.as_deref() == Some("Professional ($299/mo)")));
    }

    #[test]
    fn invalid_values_are_dropped_not_invented() {
        let row = json!({
            "id": "6bc849f1-1e20-4cd3-9a75-fc90964e911d",
            "name": "",
            "created_at": "2026-09-30T10:00:00+00:00",
            "email_addresses": { "email": "not an email" }
        });
        let lead = capture_to_lead(&row, None);
        assert_eq!(lead.email_from, None);
        assert_eq!(lead.name, "not an email");
        let mut l = empty_lead("x".into(), "y".repeat(400), "t".into());
        l.website = Some("ftp thing".into());
        l.title = Some("Dr".into());
        l.function = Some("z".repeat(200));
        let l = conform(l);
        assert_eq!(l.name.chars().count(), 255);
        assert_eq!(l.website, None);
        assert_eq!(l.title, None);
        assert_eq!(l.function.unwrap().chars().count(), 128);
    }

    #[test]
    fn every_stage_code_is_unique_per_pipeline_and_ids_are_unique() {
        let mut ids: Vec<&str> = STAGES.iter().map(|s| s.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), STAGES.len());
        assert_eq!(stage_in_team("nope", TEAM_SALES).code, "new");
        assert_eq!(stage_in_team("", TEAM_RECRUITMENT).code, "applied");
    }
}
