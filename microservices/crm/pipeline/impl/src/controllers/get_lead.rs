// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_pipeline_gen::handlers::get_lead::Request;
use serde_json::{json, Value};

#[handler(GetLeadController)]
pub fn handle(req: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    if let Err(denied) = crate::auth::require_viewer(req.jwt_claims.as_ref()) {
        return denied;
    }
    match crate::supabase::fetch_lead(&req.data.id) {
        Ok(Some(lead)) => HttpJson::new(200, json!(lead)),
        Ok(None) => HttpJson::new(404, json!({ "code": 404, "message": "lead not found" })),
        Err(error) => HttpJson::new(502, json!({ "code": 502, "message": error })),
    }
}
