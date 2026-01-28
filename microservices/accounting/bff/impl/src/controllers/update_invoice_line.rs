// User-owned controller for handler 'update_invoice_line'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::update_invoice_line::{Request, Response};

#[handler(UpdateInvoiceLineController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "created_at": "2024-01-15T09:00:00Z",
    //   "currency_code": "USD",
    //   "discount_amount": rust_decimal::Decimal::new(5625, 1),
    //   "discount_percent": rust_decimal::Decimal::new(5, 0),
    //   "id": "a0020e8400-e29b-41d4-a716-446655440000",
    //   "invoice_id": "a0010e8400-e29b-41d4-a716-446655440000",
    //   "line_number": 1,
    //   "line_subtotal": rust_decimal::Decimal::new(106875, 1),
    //   "line_total": rust_decimal::Decimal::new(1175625, 2),
    //   "product_name": "Professional Services",
    //   "quantity": rust_decimal::Decimal::new(45, 0),
    //   "tax_amount": rust_decimal::Decimal::new(106875, 2),
    //   "unit_price": rust_decimal::Decimal::new(250, 0),
    //   "updated_at": "2024-01-15T11:00:00Z"
    // }

    Response {
        account_id: Some("example".to_string()),
        created_at: Some("2024-01-15T09:00:00Z".to_string()),
        currency_code: "USD".to_string(),
        discount_amount: Some(rust_decimal::Decimal::new(5625, 1)),
        discount_percent: Some(rust_decimal::Decimal::new(5, 0)),
        id: "a0020e8400-e29b-41d4-a716-446655440000".to_string(),
        invoice_id: "a0010e8400-e29b-41d4-a716-446655440000".to_string(),
        line_number: Some(1),
        line_subtotal: Some(rust_decimal::Decimal::new(106875, 1)),
        line_total: Some(rust_decimal::Decimal::new(1175625, 2)),
        metadata: Some(Default::default()),
        product_code: Some("example".to_string()),
        product_description: Some("example".to_string()),
        product_id: Some("example".to_string()),
        product_name: "Professional Services".to_string(),
        quantity: rust_decimal::Decimal::new(45, 0),
        tax_amount: Some(rust_decimal::Decimal::new(106875, 2)),
        tax_id: Some("example".to_string()),
        tax_rate: Some(rust_decimal::Decimal::new(314, 2)),
        unit_of_measure: Some("example".to_string()),
        unit_price: rust_decimal::Decimal::new(250, 0),
        updated_at: Some("2024-01-15T11:00:00Z".to_string()),
    }
}
