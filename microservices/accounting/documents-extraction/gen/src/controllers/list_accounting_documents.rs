// User-owned controller for handler 'list_accounting_documents'.

use crate::handlers::list_accounting_documents::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AccountingDocument;

#[handler(ListAccountingDocumentsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        items: Some(vec![]),
        limit: 42,
        page: 42,
        total: 42,
    })
}
