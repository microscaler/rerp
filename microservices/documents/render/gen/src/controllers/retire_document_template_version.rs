// User-owned controller for handler 'retire_document_template_version'.

use crate::handlers::retire_document_template_version::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::TemplateStatus;

#[handler(RetireDocumentTemplateVersionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        bundle_sha256: Some("example".to_string()),
        created_at: "example".to_string(),
        engine: "example".to_string(),
        id: "example".to_string(),
        published_at: Some("example".to_string()),
        render_schema: "example".to_string(),
        retired_at: Some("example".to_string()),
        status: Default::default(),
        template_contract: "example".to_string(),
        template_id: "example".to_string(),
        version_number: 42,
    }
}
