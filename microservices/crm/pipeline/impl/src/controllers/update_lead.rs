// BRRTRouter: user-owned
//
// The just-enough CRM stores exactly two mutable things per lead: its triage
// stage and a free-text note (surfaced as `description`). Everything else on
// the wire Lead is a live projection of the marketing DB, so update requests
// that only touch other fields are rejected honestly rather than silently
// dropped.

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_pipeline_gen::handlers::update_lead::Request;
use serde_json::{json, Value};

#[handler(UpdateLeadController)]
pub fn handle(req: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    if let Err(denied) = crate::auth::require_editor(req.jwt_claims.as_ref()) {
        return denied;
    }
    let data = req.data;
    let stage_code = match data.stage_id.as_deref() {
        Some(id) => match crate::supabase::stage_by_id(id) {
            Some(def) => Some(def.code),
            None => {
                return HttpJson::new(400, json!({ "code": 400, "message": "unknown stage_id" }));
            }
        },
        None => None,
    };
    let note = data.description.as_deref();
    if stage_code.is_none() && note.is_none() {
        return HttpJson::new(
            400,
            json!({
                "code": 400,
                "message": "only stage_id and description (triage note) are updatable in the just-enough CRM"
            }),
        );
    }
    if let Err(error) = crate::supabase::write_state(&data.id, stage_code, note) {
        return HttpJson::new(502, json!({ "code": 502, "message": error }));
    }
    match crate::supabase::fetch_lead(&data.id) {
        Ok(Some(lead)) => HttpJson::new(200, json!(lead)),
        Ok(None) => HttpJson::new(404, json!({ "code": 404, "message": "lead not found" })),
        Err(error) => HttpJson::new(502, json!({ "code": 502, "message": error })),
    }
}
