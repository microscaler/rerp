// User-owned controller for handler 'delete_edi_document'.
use crate::handlers::delete_edi_document::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteEdiDocumentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
