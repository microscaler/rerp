// User-owned controller for handler 'update_payment_term'.

use crate::handlers::update_payment_term::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdatePaymentTermController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
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
    })
}
