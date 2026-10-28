// User-owned controller for handler 'list_tax_audit_packs'.

use crate::handlers::list_tax_audit_packs::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::TaxAuditPack;

#[handler(ListTaxAuditPacksController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: Some(vec![]),
        limit: 42,
        page: 42,
        total: 42,
    }
}
