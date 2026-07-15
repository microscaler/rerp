// User-owned controller for handler 'create_approval'.

use crate::handlers::create_approval::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateApprovalController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        action: "example".to_string(),
        approver_id: "example".to_string(),
        date: "example".to_string(),
        id: "example".to_string(),
        invoice_id: "example".to_string(),
        notes: Some("example".to_string()),
        threshold_met: Some(3.14),
    })
}
