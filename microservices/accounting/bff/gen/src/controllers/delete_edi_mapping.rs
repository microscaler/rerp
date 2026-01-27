// User-owned controller for handler 'delete_edi_mapping'.
use crate::handlers::delete_edi_mapping::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteEdiMappingController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
