// User-owned controller for handler 'update_customer_invoice'.
use crate::handlers::update_customer_invoice::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateCustomerInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "aging_bucket": "1-30",
    //   "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //   "created_at": "2024-01-15T09:00:00Z",
    //   "currency_code": "USD",
    //   "customer_id": "111e8400-e29b-41d4-a716-446655440001",
    //   "id": "a0030e8400-e29b-41d4-a716-446655440000",
    //   "invoice_id": "a0010e8400-e29b-41d4-a716-446655440000",
    //   "original_amount": 11000.0,
    //   "outstanding_amount": 5500.0,
    //   "status": "PARTIAL",
    //   "updated_at": "2024-01-15T11:00:00Z"
    // }

    Response {
        aging_bucket: Some("1-30".to_string()),
        collection_status: Some("example".to_string()),
        created_at: Some("2024-01-15T09:00:00Z".to_string()),
        credit_limit: Some(rust_decimal::Decimal::new(12345, 2)),
        credit_used: Some(rust_decimal::Decimal::new(12345, 2)),
        customer_id: "111e8400-e29b-41d4-a716-446655440001".to_string(),
        days_overdue: Some(42),
        id: "a0030e8400-e29b-41d4-a716-446655440000".to_string(),
        invoice_id: "a0010e8400-e29b-41d4-a716-446655440000".to_string(),
        last_payment_amount: Some(rust_decimal::Decimal::new(12345, 2)),
        last_payment_date: Some("example".to_string()),
        metadata: Some(Default::default()),
        outstanding_amount: Some(rust_decimal::Decimal::new(55000, 1)),
        updated_at: Some("2024-01-15T11:00:00Z".to_string()),
        write_off_amount: Some(rust_decimal::Decimal::new(12345, 2)),
        write_off_date: Some("example".to_string()),
        write_off_reason: Some("example".to_string()),
    }
}
