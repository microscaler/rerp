// User-owned controller for handler 'approve_data_request'.

use crate::handlers::approve_data_request::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ApproveDataRequestController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        completed_at: Some("example".to_string()),
        contact_email: Some("example".to_string()),
        created_at: "example".to_string(),
        id: "example".to_string(),
        kind: "example".to_string(),
        note: Some("example".to_string()),
        requested_by: Some("example".to_string()),
        source: Some("example".to_string()),
        status: "example".to_string(),
        steps: Some(vec![]),
        subject_hash: Some("example".to_string()),
    })
}
