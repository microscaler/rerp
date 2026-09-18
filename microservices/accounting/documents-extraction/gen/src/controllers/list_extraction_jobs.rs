// User-owned controller for handler 'list_extraction_jobs'.

use crate::handlers::list_extraction_jobs::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListExtractionJobsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
