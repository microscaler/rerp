// Implementation stub for handler 'list_leases'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_lease_accounting_gen::handlers::list_leases::{Request, Response};

#[handler(ListLeasesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
    }
}
