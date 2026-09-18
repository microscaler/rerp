// Implementation stub for handler 'submit_tax_return'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_tax_compliance_gen::handlers::submit_tax_return::{Request, Response};

#[handler(SubmitTaxReturnController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        external_reference: None,
        id: "".to_string(),
        return_id: "".to_string(),
        status: "".to_string(),
    }
}
