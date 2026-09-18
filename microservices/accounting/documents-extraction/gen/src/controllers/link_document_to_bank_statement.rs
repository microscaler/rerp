// User-owned controller for handler 'link_document_to_bank_statement'.

use crate::handlers::link_document_to_bank_statement::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(LinkDocumentToBankStatementController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        document_id: "example".to_string(),
        id: "example".to_string(),
        target_id: "example".to_string(),
        target_type: "example".to_string(),
    }
}
