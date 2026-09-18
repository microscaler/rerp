// User-owned controller for handler 'preview_document_template_version'.

use crate::handlers::preview_document_template_version::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(PreviewDocumentTemplateVersionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        download_url: "example".to_string(),
        expires_at: "example".to_string(),
        media_type: "example".to_string(),
        sha256: "example".to_string(),
        size_bytes: 42,
    }
}
