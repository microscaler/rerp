// User-owned controller for handler 'list_accounting_documents'.

use crate::handlers::list_accounting_documents::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListAccountingDocumentsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
