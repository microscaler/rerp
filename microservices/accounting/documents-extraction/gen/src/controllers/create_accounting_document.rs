// User-owned controller for handler 'create_accounting_document'.

use crate::handlers::create_accounting_document::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateAccountingDocumentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        company_id: Some("example".to_string()),
        content_type: Some("example".to_string()),
        document_type: Default::default(),
        file_name: "example".to_string(),
        id: "example".to_string(),
        source_uri: Some("example".to_string()),
        status: Default::default(),
        uploaded_at: "example".to_string(),
    })
}
