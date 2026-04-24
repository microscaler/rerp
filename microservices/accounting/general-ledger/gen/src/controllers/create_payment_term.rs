// User-owned controller for handler 'create_payment_term'.

use crate::handlers::create_payment_term::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreatePaymentTermController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        code: "example".to_string(),
        company_id: "example".to_string(),
        created_at: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        description: Some("example".to_string()),
        id: "example".to_string(),
        is_active: true,
        name: "example".to_string(),
        r#type: "example".to_string(),
        updated_at: Some("example".to_string()),
    }
}
