// Implementation stub for handler 'list_extraction_jobs'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_documents_extraction_gen::handlers::list_extraction_jobs::{Request, Response};

#[handler(ListExtractionJobsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
    }
}
