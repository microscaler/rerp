// User-owned controller for handler 'list_document_templates'.

use crate::handlers::list_document_templates::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DocumentTemplate;

#[handler(ListDocumentTemplatesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: vec![],
        limit: 42,
        page: 42,
        total: 42,
    }
}
