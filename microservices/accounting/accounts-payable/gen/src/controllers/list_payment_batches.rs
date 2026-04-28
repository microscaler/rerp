// User-owned controller for handler 'list_payment_batches'.

use crate::handlers::list_payment_batches::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::PaymentBatch;

#[handler(ListPaymentBatchesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: vec![],
        total: 42,
    }
}
