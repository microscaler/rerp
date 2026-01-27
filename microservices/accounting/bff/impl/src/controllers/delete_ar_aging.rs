// User-owned controller for handler 'delete_ar_aging'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::delete_ar_aging::{Request, Response};

#[handler(DeleteArAgingController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
