// User-owned controller for handler 'create_revaluation'.

use crate::handlers::create_revaluation::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateRevaluationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        asset_id: "example".to_string(),
        created_at: Some("example".to_string()),
        created_by: Some("example".to_string()),
        gl_entry_id: Some("example".to_string()),
        id: "example".to_string(),
        new_value: 3.14,
        previous_value: 3.14,
        reason: Some("example".to_string()),
        revaluation_date: "example".to_string(),
    }
}
