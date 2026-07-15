// User-owned controller for handler 'list_document_template_versions'.

use crate::handlers::list_document_template_versions::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DocumentTemplateVersion;

#[handler(ListDocumentTemplateVersionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: vec![],
        limit: 42,
        page: 42,
        total: 42,
    }
}
