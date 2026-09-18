// User-owned controller for handler 'validate_document_template_version'.

use crate::handlers::validate_document_template_version::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ValidationIssue;

#[handler(ValidateDocumentTemplateVersionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        checked_at: "example".to_string(),
        issues: vec![],
        valid: true,
    }
}
