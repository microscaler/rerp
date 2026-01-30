// User-owned controller for handler 'create_invoice_line'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_invoice_gen::handlers::create_invoice_line::{Request, Response};
use rust_decimal::Decimal;

#[handler(CreateInvoiceLineController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "created_at": "2024-01-15T09:00:00Z",
    //   "currency_code": "USD",
    //   "discount_amount": 0.0,
    //   "discount_percent": 0.0,
    //   "id": "a0020e8400-e29b-41d4-a716-446655440000",
    //   "invoice_id": "a0010e8400-e29b-41d4-a716-446655440000",
    //   "line_number": 1,
    //   "line_subtotal": 10000.0,
    //   "line_total": 10000.0,
    //   "product_name": "Professional Services",
    //   "quantity": 40.0,
    //   "tax_amount": 0.0,
    //   "unit_price": 250.0,
    //   "updated_at": "2024-01-15T09:00:00Z"
    // }

    Response {
        account_id: Some("example".to_string()),
        created_at: Some("2024-01-15T09:00:00Z".to_string()),
        currency_code: "USD".to_string(),
        discount_amount: Some(Decimal::ZERO),
        discount_percent: Some(Decimal::ZERO),
        id: "a0020e8400-e29b-41d4-a716-446655440000".to_string(),
        invoice_id: "a0010e8400-e29b-41d4-a716-446655440000".to_string(),
        line_number: Some(1),
        line_subtotal: Some(Decimal::new(10000, 0)),
        line_total: Some(Decimal::new(10000, 0)),
        metadata: Some(Default::default()),
        product_code: Some("example".to_string()),
        product_description: Some("example".to_string()),
        product_id: Some("example".to_string()),
        product_name: "Professional Services".to_string(),
        quantity: Decimal::new(40, 0),
        tax_amount: Some(Decimal::ZERO),
        tax_id: Some("example".to_string()),
        tax_rate: Some(Decimal::new(314, 2)),
        unit_of_measure: Some("example".to_string()),
        unit_price: Decimal::new(250, 0),
        updated_at: Some("2024-01-15T09:00:00Z".to_string()),
    }
}
