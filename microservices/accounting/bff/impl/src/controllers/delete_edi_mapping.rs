// User-owned controller for handler 'delete_edi_mapping'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::delete_edi_mapping::{Request, Response};

#[handler(DeleteEdiMappingController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
