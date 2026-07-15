// User-owned controller for handler 'create_line_item'.

use crate::handlers::create_line_item::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateLineItemController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        amount: 3.14,
        created_at: Some("example".to_string()),
        description: "example".to_string(),
        discount_amount: 3.14,
        discount_percent: 3.14,
        gl_account_credit: "example".to_string(),
        gl_account_debit: "example".to_string(),
        id: "example".to_string(),
        invoice_id: "example".to_string(),
        product_code: "example".to_string(),
        product_id: "example".to_string(),
        product_name: "example".to_string(),
        quantity: 3.14,
        tax_amount: 3.14,
        tax_code: "example".to_string(),
        tax_rate: 3.14,
        unit_price: 3.14,
        updated_at: Some("example".to_string()),
    })
}
