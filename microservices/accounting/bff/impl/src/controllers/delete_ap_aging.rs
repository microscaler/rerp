// User-owned controller for handler 'delete_ap_aging'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::delete_ap_aging::{Request, Response};

#[handler(DeleteApAgingController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
