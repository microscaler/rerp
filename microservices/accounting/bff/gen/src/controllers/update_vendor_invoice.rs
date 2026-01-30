// User-owned controller for handler 'update_vendor_invoice'.
use crate::handlers::update_vendor_invoice::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateVendorInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "aging_bucket": "1-30",
    //   "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //   "created_at": "2024-01-15T09:00:00Z",
    //   "currency_code": "USD",
    //   "id": "a0050e8400-e29b-41d4-a716-446655440000",
    //   "invoice_id": "a0011e8400-e29b-41d4-a716-446655440001",
    //   "original_amount": 15000.0,
    //   "outstanding_amount": 7500.0,
    //   "status": "PARTIAL",
    //   "updated_at": "2024-01-15T11:00:00Z",
    //   "vendor_id": "411e8400-e29b-41d4-a716-446655440001"
    // }

    Response {
        aging_bucket: Some("1-30".to_string()),
        approval_status: Some("example".to_string()),
        approved_at: Some("example".to_string()),
        approved_by: Some("example".to_string()),
        created_at: Some("2024-01-15T09:00:00Z".to_string()),
        days_until_due: Some(42),
        early_payment_discount_date: Some("example".to_string()),
        early_payment_discount_percent: Some(rust_decimal::Decimal::new(12345, 2)),
        id: "a0050e8400-e29b-41d4-a716-446655440000".to_string(),
        invoice_id: "a0011e8400-e29b-41d4-a716-446655440001".to_string(),
        matching_status: Some("example".to_string()),
        metadata: Some(Default::default()),
        outstanding_amount: Some(rust_decimal::Decimal::new(75000, 1)),
        purchase_order_id: Some("example".to_string()),
        updated_at: Some("2024-01-15T11:00:00Z".to_string()),
        vendor_id: "411e8400-e29b-41d4-a716-446655440001".to_string(),
    }
}
