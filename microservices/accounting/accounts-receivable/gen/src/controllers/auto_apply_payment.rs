// User-owned controller for handler 'auto_apply_payment'.

use crate::handlers::auto_apply_payment::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(AutoApplyPaymentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        applications: vec![],
        payment_id: "example".to_string(),
        total_applied: 3.14,
        unapplied_amount: 3.14,
    })
}
