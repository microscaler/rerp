// User-owned controller for handler 'list_extraction_jobs'.

use crate::handlers::list_extraction_jobs::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ExtractionJob;

#[handler(ListExtractionJobsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: Some(vec![]),
        limit: 42,
        page: 42,
        total: 42,
    }
}
