// User-owned controller for handler 'update_invoice'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_invoice_gen::handlers::update_invoice::{Request, Response};

#[handler(UpdateInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //   "created_at": "2024-01-15T09:00:00Z",
    //   "currency_code": "USD",
    //   "customer_id": "111e8400-e29b-41d4-a716-446655440001",
    //   "discount_amount": rust_decimal::Decimal::new(0, 0),
    //   "due_date": "2024-02-15",
    //   "exchange_rate": rust_decimal::Decimal::new(1, 0),
    //   "id": "a0010e8400-e29b-41d4-a716-446655440000",
    //   "invoice_date": "2024-01-15",
    //   "invoice_number": "INV-2024-001",
    //   "invoice_type": "CUSTOMER_INVOICE",
    //   "notes": "Updated invoice notes",
    //   "outstanding_amount": rust_decimal::Decimal::new(11000, 0),
    //   "paid_amount": rust_decimal::Decimal::new(0, 0),
    //   "payment_state": "NOT_PAID",
    //   "posted_at": "2024-01-15T11:00:00Z",
    //   "status": "POSTED",
    //   "subtotal": rust_decimal::Decimal::new(10000, 0),
    //   "tax_amount": rust_decimal::Decimal::new(1000, 0),
    //   "total_amount": rust_decimal::Decimal::new(11000, 0),
    //   "updated_at": "2024-01-15T11:00:00Z"
    // }

    Response {
        cancelled_at: Some("example".to_string()),
        company_id: Some("550e8400-e29b-41d4-a716-446655440000".to_string()),
        created_at: Some("2024-01-15T09:00:00Z".to_string()),
        currency_code: "USD".to_string(),
        customer_id: Some("111e8400-e29b-41d4-a716-446655440001".to_string()),
        discount_amount: Some(rust_decimal::Decimal::new(0, 0)),
        due_date: Some("2024-02-15".to_string()),
        exchange_rate: Some(rust_decimal::Decimal::new(1, 0)),
        id: "a0010e8400-e29b-41d4-a716-446655440000".to_string(),
        internal_notes: Some("example".to_string()),
        invoice_date: "2024-01-15".to_string(),
        invoice_number: "INV-2024-001".to_string(),
        invoice_type: "CUSTOMER_INVOICE".to_string(),
        metadata: Some(Default::default()),
        notes: Some("Updated invoice notes".to_string()),
        outstanding_amount: Some(rust_decimal::Decimal::new(11000, 0)),
        paid_amount: Some(rust_decimal::Decimal::new(0, 0)),
        paid_at: Some("example".to_string()),
        payment_state: "NOT_PAID".to_string(),
        payment_term_id: Some("example".to_string()),
        posted_at: Some("2024-01-15T11:00:00Z".to_string()),
        reference_number: Some("example".to_string()),
        status: "POSTED".to_string(),
        subtotal: Some(rust_decimal::Decimal::new(10000, 0)),
        tax_amount: Some(rust_decimal::Decimal::new(1000, 0)),
        total_amount: Some(rust_decimal::Decimal::new(11000, 0)),
        updated_at: Some("2024-01-15T11:00:00Z".to_string()),
        vendor_id: Some("example".to_string()),
        vendor_reference: Some("example".to_string()),
    }
}
