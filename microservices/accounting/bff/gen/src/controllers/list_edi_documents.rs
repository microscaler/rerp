// User-owned controller for handler 'list_edi_documents'.
use crate::handlers::list_edi_documents::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::EdiDocument;

#[handler(ListEdiDocumentsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: Some(Default::default()),
        limit: Some(42),
        page: Some(42),
        total: Some(42),
    }
}
