// BRRTRouter: user-owned
//
// The CRM owns exactly three things per lead: its stage (within the lead's
// pipeline), its priority (Odoo stars: LOW/NORMAL/HIGH/URGENT = 0-3) and a
// free-text triage note (`description`). Everything else on the wire Lead is
// a live projection of the marketing DB, so a request that only touches other
// fields is rejected honestly rather than silently dropped.

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_pipeline_gen::handlers::update_lead::Request;
use serde_json::{json, Value};

use crate::supabase::{fetch_lead, priority_stars, stage_for_lead, write_state, StateChange};

#[handler(UpdateLeadController)]
pub fn handle(req: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    if let Err(denied) = crate::auth::require_editor(req.jwt_claims.as_ref()) {
        return denied;
    }
    let data = req.data;
    if data.stage_id.is_none() && data.description.is_none() && data.priority.is_none() {
        return HttpJson::new(
            400,
            json!({
                "code": 400,
                "message": "only stage_id, priority and description (triage note) are updatable in the just-enough CRM"
            }),
        );
    }
    let lead = match fetch_lead(&data.id) {
        Ok(Some(lead)) => lead,
        Ok(None) => return HttpJson::new(404, json!({ "code": 404, "message": "lead not found" })),
        Err(error) => return HttpJson::new(502, json!({ "code": 502, "message": error })),
    };
    let stage = match data.stage_id.as_deref() {
        Some(id) => match stage_for_lead(&lead, id) {
            Ok(def) => Some(def.code),
            Err(message) => return HttpJson::new(400, json!({ "code": 400, "message": message })),
        },
        None => None,
    };
    let priority = match data.priority.as_deref() {
        Some(p) => match priority_stars(p) {
            Some(stars) => Some(stars),
            None => return HttpJson::new(400, json!({ "code": 400, "message": "unknown priority" })),
        },
        None => None,
    };
    let change = StateChange { stage, note: data.description.as_deref(), priority };
    if let Err(error) = write_state(&lead, change) {
        return HttpJson::new(502, json!({ "code": 502, "message": error }));
    }
    match fetch_lead(&data.id) {
        Ok(Some(lead)) => HttpJson::new(200, json!(lead)),
        Ok(None) => HttpJson::new(404, json!({ "code": 404, "message": "lead not found" })),
        Err(error) => HttpJson::new(502, json!({ "code": 502, "message": error })),
    }
}
