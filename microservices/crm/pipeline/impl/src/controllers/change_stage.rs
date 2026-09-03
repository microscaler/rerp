// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_pipeline_gen::handlers::change_stage::Request;
use serde_json::{json, Value};

#[handler(ChangeStageController)]
pub fn handle(req: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    if let Err(denied) = crate::auth::require_editor(req.jwt_claims.as_ref()) {
        return denied;
    }
    let data = req.data;
    let def = match crate::supabase::stage_by_id(&data.stage_id) {
        Some(def) => def,
        None => {
            return HttpJson::new(400, json!({ "code": 400, "message": "unknown stage_id" }));
        }
    };
    // An optional comment on the transition becomes the triage note.
    let note = data.comment.as_deref().filter(|c| !c.is_empty());
    if let Err(error) = crate::supabase::write_state(&data.id, Some(def.code), note) {
        return HttpJson::new(502, json!({ "code": 502, "message": error }));
    }
    match crate::supabase::fetch_lead(&data.id) {
        Ok(Some(lead)) => HttpJson::new(200, json!(lead)),
        Ok(None) => HttpJson::new(404, json!({ "code": 404, "message": "lead not found" })),
        Err(error) => HttpJson::new(502, json!({ "code": 502, "message": error })),
    }
}
