// User-owned controller for handler 'create_tax_audit_pack'.

use crate::handlers::create_tax_audit_pack::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateTaxAuditPackController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        artifact_count: 42,
        id: "example".to_string(),
        return_id: "example".to_string(),
        status: "example".to_string(),
    }
}
