// User-owned controller for handler 'list_lease_payment_schedules'.

use crate::handlers::list_lease_payment_schedules::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListLeasePaymentSchedulesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
