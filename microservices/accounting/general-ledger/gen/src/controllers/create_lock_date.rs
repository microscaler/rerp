// User-owned controller for handler 'create_lock_date'.

use crate::handlers::create_lock_date::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateLockDateController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        company_id: "example".to_string(),
        id: "example".to_string(),
        lock_type: "example".to_string(),
        locked_through_date: "example".to_string(),
        reason: Some("example".to_string()),
    }
}
