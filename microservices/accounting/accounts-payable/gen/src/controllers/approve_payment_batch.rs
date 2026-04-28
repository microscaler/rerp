// User-owned controller for handler 'approve_payment_batch'.

use crate::handlers::approve_payment_batch::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ApprovePaymentBatchController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
