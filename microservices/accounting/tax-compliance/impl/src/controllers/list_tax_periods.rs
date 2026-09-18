// Implementation stub for handler 'list_tax_periods'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_tax_compliance_gen::handlers::list_tax_periods::{Request, Response};

#[handler(ListTaxPeriodsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
    }
}
