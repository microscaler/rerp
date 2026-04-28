// User-owned controller for handler 'list_audit_events'.

use crate::handlers::list_audit_events::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListAuditEventsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
