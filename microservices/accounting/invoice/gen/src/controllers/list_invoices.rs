// User-owned controller for handler 'list_invoices'.

use crate::handlers::list_invoices::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Invoice;

#[handler(ListInvoicesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        has_more: Some(true),
        items: vec![],
        limit: 42,
        page: 42,
        total: 42,
    })
}
