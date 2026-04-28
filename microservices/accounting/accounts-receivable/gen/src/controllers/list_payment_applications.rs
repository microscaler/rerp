// User-owned controller for handler 'list_payment_applications'.

use crate::handlers::list_payment_applications::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::PaymentApplication;

#[handler(ListPaymentApplicationsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response(vec![])
}
