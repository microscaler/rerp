// Implementation stub for handler 'create_lease_payment_schedule'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_lease_accounting_gen::handlers::create_lease_payment_schedule::{Request, Response};

#[handler(CreateLeasePaymentScheduleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        amount: 0.0,
        currency_code: None,
        id: "".to_string(),
        lease_id: "".to_string(),
        payment_date: "".to_string(),
    }
}
