// User-owned controller for handler 'classify_accounting_document'.

use crate::handlers::classify_accounting_document::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ClassifyAccountingDocumentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        confidence: 3.14,
        document_id: "example".to_string(),
        document_type: Default::default(),
        reasons: Some(vec![]),
    }
}
