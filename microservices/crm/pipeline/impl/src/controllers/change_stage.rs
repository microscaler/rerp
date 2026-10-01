// BRRTRouter: user-owned
//
// A card dropped on another column. The stage must belong to the lead's own
// pipeline (an applicant never lands in a sales column); an optional comment
// becomes the triage note.

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_pipeline_gen::handlers::change_stage::Request;
use serde_json::{json, Value};

use crate::supabase::{fetch_lead, stage_for_lead, write_state, StateChange};

#[handler(ChangeStageController)]
pub fn handle(req: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    if let Err(denied) = crate::auth::require_editor(req.jwt_claims.as_ref()) {
        return denied;
    }
    let data = req.data;
    let lead = match fetch_lead(&data.id) {
        Ok(Some(lead)) => lead,
        Ok(None) => return HttpJson::new(404, json!({ "code": 404, "message": "lead not found" })),
        Err(error) => return HttpJson::new(502, json!({ "code": 502, "message": error })),
    };
    let def = match stage_for_lead(&lead, &data.stage_id) {
        Ok(def) => def,
        Err(message) => return HttpJson::new(400, json!({ "code": 400, "message": message })),
    };
    let change = StateChange {
        stage: Some(def.code),
        note: data.comment.as_deref().filter(|c| !c.is_empty()),
        priority: None,
    };
    if let Err(error) = write_state(&lead, change) {
        return HttpJson::new(502, json!({ "code": 502, "message": error }));
    }
    match fetch_lead(&data.id) {
        Ok(Some(lead)) => HttpJson::new(200, json!(lead)),
        Ok(None) => HttpJson::new(404, json!({ "code": 404, "message": "lead not found" })),
        Err(error) => HttpJson::new(502, json!({ "code": 502, "message": error })),
    }
}
