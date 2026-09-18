// User-owned controller for handler 'get_document_template'.

use crate::handlers::get_document_template::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetDocumentTemplateController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        created_at: "example".to_string(),
        description: Some("example".to_string()),
        document_kind: "example".to_string(),
        id: "example".to_string(),
        key: "example".to_string(),
        name: "example".to_string(),
        published_version_id: Some("example".to_string()),
        updated_at: Some("example".to_string()),
    }
}
