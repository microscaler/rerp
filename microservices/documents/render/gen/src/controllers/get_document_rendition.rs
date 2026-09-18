// User-owned controller for handler 'get_document_rendition'.

use crate::handlers::get_document_rendition::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DocumentMutability;
#[allow(unused_imports)]
use crate::handlers::types::DocumentOrigin;
#[allow(unused_imports)]
use crate::handlers::types::RenditionRole;
#[allow(unused_imports)]
use crate::handlers::types::RenditionStatus;
#[allow(unused_imports)]
use crate::handlers::types::SourceReference;

#[handler(GetDocumentRenditionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        created_at: "example".to_string(),
        document_id: Some("example".to_string()),
        failure_code: Some("example".to_string()),
        failure_message: Some("example".to_string()),
        id: "example".to_string(),
        media_type: "example".to_string(),
        mutability: Default::default(),
        origin: Default::default(),
        ready_at: Some("example".to_string()),
        renderer: Some("example".to_string()),
        renderer_version: Some("example".to_string()),
        role: Default::default(),
        sha256: Some("example".to_string()),
        size_bytes: Some(42),
        source: Default::default(),
        source_rendition_id: Some("example".to_string()),
        status: Default::default(),
        template_version_id: Some("example".to_string()),
    }
}
