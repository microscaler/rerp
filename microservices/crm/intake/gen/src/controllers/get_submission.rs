// User-owned controller for handler 'get_submission'.

use crate::handlers::get_submission::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetSubmissionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        confirmation_count: Some(42),
        confirmation_sent_at: Some("example".to_string()),
        confirmed_at: Some("example".to_string()),
        decided_at: Some("example".to_string()),
        email_masked: Some("example".to_string()),
        expires_at: Some("example".to_string()),
        form_id: Some("example".to_string()),
        form_name: Some("example".to_string()),
        id: "example".to_string(),
        kind: "example".to_string(),
        reject_reason: Some("example".to_string()),
        status: "example".to_string(),
        submitted_at: "example".to_string(),
    })
}
