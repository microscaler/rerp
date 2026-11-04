// User-owned controller for handler 'create_document_rendition_download'.

use crate::handlers::create_document_rendition_download::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateDocumentRenditionDownloadController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        expires_at: "example".to_string(),
        url: "example".to_string(),
    })
}
