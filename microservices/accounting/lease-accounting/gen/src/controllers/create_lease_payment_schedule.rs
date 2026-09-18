// User-owned controller for handler 'create_lease_payment_schedule'.

use crate::handlers::create_lease_payment_schedule::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateLeasePaymentScheduleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        amount: 3.14,
        currency_code: Some("example".to_string()),
        id: "example".to_string(),
        lease_id: "example".to_string(),
        payment_date: "example".to_string(),
    }
}
