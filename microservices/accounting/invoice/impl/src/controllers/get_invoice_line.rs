// User-owned controller for handler 'get_invoice_line'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_invoice_gen::handlers::get_invoice_line::{Request, Response};

#[handler(GetInvoiceLineController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "created_at": "2024-01-15T09:00:00Z",
    //   "currency_code": "USD",
    //   "discount_amount": rust_decimal::Decimal::new(0, 0),
    //   "discount_percent": rust_decimal::Decimal::new(0, 0),
    //   "id": "a0020e8400-e29b-41d4-a716-446655440000",
    //   "invoice_id": "a0010e8400-e29b-41d4-a716-446655440000",
    //   "line_number": 1,
    //   "line_subtotal": rust_decimal::Decimal::new(10000, 0),
    //   "line_total": rust_decimal::Decimal::new(11000, 0),
    //   "product_name": "Professional Services",
    //   "quantity": rust_decimal::Decimal::new(40, 0),
    //   "tax_amount": rust_decimal::Decimal::new(1000, 0),
    //   "unit_price": rust_decimal::Decimal::new(250, 0),
    //   "updated_at": "2024-01-15T09:00:00Z"
    // }

    Response {
        account_id: Some("example".to_string()),
        created_at: Some("2024-01-15T09:00:00Z".to_string()),
        currency_code: "USD".to_string(),
        discount_amount: Some(rust_decimal::Decimal::new(0, 0)),
        discount_percent: Some(rust_decimal::Decimal::new(0, 0)),
        id: "a0020e8400-e29b-41d4-a716-446655440000".to_string(),
        invoice_id: "a0010e8400-e29b-41d4-a716-446655440000".to_string(),
        line_number: Some(1),
        line_subtotal: Some(rust_decimal::Decimal::new(10000, 0)),
        line_total: Some(rust_decimal::Decimal::new(11000, 0)),
        metadata: Some(Default::default()),
        product_code: Some("example".to_string()),
        product_description: Some("example".to_string()),
        product_id: Some("example".to_string()),
        product_name: "Professional Services".to_string(),
        quantity: rust_decimal::Decimal::new(40, 0),
        tax_amount: Some(rust_decimal::Decimal::new(1000, 0)),
        tax_id: Some("example".to_string()),
        tax_rate: Some(rust_decimal::Decimal::new(314, 2)),
        unit_of_measure: Some("example".to_string()),
        unit_price: rust_decimal::Decimal::new(250, 0),
        updated_at: Some("2024-01-15T09:00:00Z".to_string()),
    }
}
