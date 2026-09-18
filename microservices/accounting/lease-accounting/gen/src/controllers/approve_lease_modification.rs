// User-owned controller for handler 'approve_lease_modification'.

use crate::handlers::approve_lease_modification::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ApproveLeaseModificationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        effective_date: "example".to_string(),
        id: "example".to_string(),
        lease_id: "example".to_string(),
        reason: Some("example".to_string()),
        status: "example".to_string(),
    }
}
