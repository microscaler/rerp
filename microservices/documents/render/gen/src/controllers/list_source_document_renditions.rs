// User-owned controller for handler 'list_source_document_renditions'.

use crate::handlers::list_source_document_renditions::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DocumentRendition;

#[handler(ListSourceDocumentRenditionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: vec![],
        limit: 42,
        page: 42,
        total: 42,
    }
}
