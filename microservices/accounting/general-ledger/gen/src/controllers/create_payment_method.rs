// User-owned controller for handler 'create_payment_method'.

use crate::handlers::create_payment_method::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreatePaymentMethodController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
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
    })
}
