// User-owned controller for handler 'list_invoice_lines'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_invoice_gen::handlers::list_invoice_lines::{Request, Response};

#[allow(unused_imports)]
use rerp_accounting_invoice_gen::handlers::types::InvoiceLine;

#[handler(ListInvoiceLinesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "created_at": "2024-01-15T09:00:00Z",
    //       "currency_code": "USD",
    //       "discount_amount": 0.0,
    //       "discount_percent": 0.0,
    //       "id": "a0020e8400-e29b-41d4-a716-446655440000",
    //       "invoice_id": "a0010e8400-e29b-41d4-a716-446655440000",
    //       "line_number": 1,
    //       "line_subtotal": 10000.0,
    //       "line_total": 11000.0,
    //       "product_name": "Professional Services",
    //       "quantity": 40.0,
    //       "tax_amount": 1000.0,
    //       "unit_price": 250.0,
    //       "updated_at": "2024-01-15T09:00:00Z"
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 1
    // }

    Response {
        items: Some(vec![serde_json::from_value::<InvoiceLine>(serde_json::json!({"created_at":"2024-01-15T09:00:00Z","currency_code":"USD","discount_amount":0.0,"discount_percent":0.0,"id":"a0020e8400-e29b-41d4-a716-446655440000","invoice_id":"a0010e8400-e29b-41d4-a716-446655440000","line_number":1,"line_subtotal":10000.0,"line_total":11000.0,"product_name":"Professional Services","quantity":40.0,"tax_amount":1000.0,"unit_price":250.0,"updated_at":"2024-01-15T09:00:00Z"})).unwrap_or_default()]),limit: Some(20),page: Some(1),total: Some(1),
    }
}
