// Implementation stub for handler 'link_document_to_bank_statement'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_documents_extraction_gen::handlers::link_document_to_bank_statement::{Request, Response};

#[handler(LinkDocumentToBankStatementController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        document_id: "".to_string(),
        id: "".to_string(),
        target_id: "".to_string(),
        target_type: "".to_string(),
    }
}
