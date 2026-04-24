// User-owned controller for handler 'update_payment_method'.

use crate::handlers::update_payment_method::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdatePaymentMethodController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        code: "example".to_string(),
        company_id: "example".to_string(),
        created_at: Some("example".to_string()),
        description: Some("example".to_string()),
        id: "example".to_string(),
        is_active: true,
        is_payable: Some(true),
        is_receivable: Some(true),
        name: "example".to_string(),
        payment_method_type: "example".to_string(),
        updated_at: Some("example".to_string()),
    }
}
