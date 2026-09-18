// User-owned controller for handler 'list_edi_mappings'.

use crate::handlers::list_edi_mappings::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListEdiMappingsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
