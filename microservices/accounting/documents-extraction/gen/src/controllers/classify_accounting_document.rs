// User-owned controller for handler 'classify_accounting_document'.

use crate::handlers::classify_accounting_document::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ClassifyAccountingDocumentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        confidence: 1.5,
        document_id: "example".to_string(),
        document_type: Default::default(),
        reasons: Some(vec![]),
    })
}
