// User-owned controller for handler 'list_edi_documents'.

use crate::handlers::list_edi_documents::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListEdiDocumentsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
