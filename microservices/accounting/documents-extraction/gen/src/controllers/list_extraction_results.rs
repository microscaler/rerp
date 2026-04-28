// User-owned controller for handler 'list_extraction_results'.

use crate::handlers::list_extraction_results::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListExtractionResultsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
