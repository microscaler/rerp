// User-owned controller for handler 'update_invoice_line'.
use crate::handlers::update_invoice_line::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateInvoiceLineController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "created_at": "2024-01-15T09:00:00Z",
    //   "currency_code": "USD",
    //   "discount_amount": 562.5,
    //   "discount_percent": 5.0,
    //   "id": "a0020e8400-e29b-41d4-a716-446655440000",
    //   "invoice_id": "a0010e8400-e29b-41d4-a716-446655440000",
    //   "line_number": 1,
    //   "line_subtotal": 10687.5,
    //   "line_total": 11756.25,
    //   "product_name": "Professional Services",
    //   "quantity": 45.0,
    //   "tax_amount": 1068.75,
    //   "unit_price": 250.0,
    //   "updated_at": "2024-01-15T11:00:00Z"
    // }

    Response {
        account_id: Some("example".to_string()),
        created_at: Some("2024-01-15T09:00:00Z".to_string()),
        currency_code: "USD".to_string(),
        discount_amount: Some(562.5),
        discount_percent: Some(5.0),
        id: "a0020e8400-e29b-41d4-a716-446655440000".to_string(),
        invoice_id: "a0010e8400-e29b-41d4-a716-446655440000".to_string(),
        line_number: Some(1),
        line_subtotal: Some(10687.5),
        line_total: Some(11756.25),
        metadata: Some(Default::default()),
        product_code: Some("example".to_string()),
        product_description: Some("example".to_string()),
        product_id: Some("example".to_string()),
        product_name: "Professional Services".to_string(),
        quantity: 45.0,
        tax_amount: Some(1068.75),
        tax_id: Some("example".to_string()),
        tax_rate: Some(3.14),
        unit_of_measure: Some("example".to_string()),
        unit_price: 250.0,
        updated_at: Some("2024-01-15T11:00:00Z".to_string()),
    }
}
