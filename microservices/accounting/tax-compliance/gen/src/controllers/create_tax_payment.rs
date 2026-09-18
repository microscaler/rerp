// User-owned controller for handler 'create_tax_payment'.

use crate::handlers::create_tax_payment::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateTaxPaymentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        amount: 3.14,
        id: "example".to_string(),
        paid_at: Some("example".to_string()),
        return_id: "example".to_string(),
        status: Default::default(),
    }
}
