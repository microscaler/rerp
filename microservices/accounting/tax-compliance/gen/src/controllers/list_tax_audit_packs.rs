// User-owned controller for handler 'list_tax_audit_packs'.

use crate::handlers::list_tax_audit_packs::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListTaxAuditPacksController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
