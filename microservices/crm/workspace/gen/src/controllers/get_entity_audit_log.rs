// User-owned controller for handler 'get_entity_audit_log'.

use crate::handlers::get_entity_audit_log::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AuditLogEntry;

#[handler(GetEntityAuditLogController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        items: vec![],
        total: Some(42),
    })
}
