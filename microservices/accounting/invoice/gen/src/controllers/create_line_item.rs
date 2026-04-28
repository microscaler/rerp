// User-owned controller for handler 'create_line_item'.

use crate::handlers::create_line_item::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateLineItemController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        amount: 3.14,
        created_at: Some("example".to_string()),
        description: "example".to_string(),
        discount_amount: Some(3.14),
        discount_percent: Some(3.14),
        gl_account_credit: Some("example".to_string()),
        gl_account_debit: Some("example".to_string()),
        id: "example".to_string(),
        invoice_id: "example".to_string(),
        product_code: Some("example".to_string()),
        product_id: Some("example".to_string()),
        product_name: Some("example".to_string()),
        quantity: 3.14,
        tax_amount: Some(3.14),
        tax_code: Some("example".to_string()),
        tax_rate: Some(3.14),
        unit_price: 3.14,
        updated_at: Some("example".to_string()),
    }
}
