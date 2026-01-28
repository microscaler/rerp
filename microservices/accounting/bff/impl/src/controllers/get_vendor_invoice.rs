// User-owned controller for handler 'get_vendor_invoice'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::get_vendor_invoice::{Request, Response};

#[handler(GetVendorInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "aging_bucket": "CURRENT",
    //   "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //   "created_at": "2024-01-15T09:00:00Z",
    //   "currency_code": "USD",
    //   "id": "a0050e8400-e29b-41d4-a716-446655440000",
    //   "invoice_id": "a0011e8400-e29b-41d4-a716-446655440001",
    //   "original_amount": rust_decimal::Decimal::new(15000, 0),
    //   "outstanding_amount": rust_decimal::Decimal::new(15000, 0),
    //   "status": "OUTSTANDING",
    //   "updated_at": "2024-01-15T09:00:00Z",
    //   "vendor_id": "411e8400-e29b-41d4-a716-446655440001"
    // }

    Response {
        aging_bucket: Some("CURRENT".to_string()),
        approval_status: Some("example".to_string()),
        approved_at: Some("example".to_string()),
        approved_by: Some("example".to_string()),
        created_at: Some("2024-01-15T09:00:00Z".to_string()),
        days_until_due: Some(42),
        early_payment_discount_date: Some("example".to_string()),
        early_payment_discount_percent: Some(rust_decimal::Decimal::new(314, 2)),
        id: "a0050e8400-e29b-41d4-a716-446655440000".to_string(),
        invoice_id: "a0011e8400-e29b-41d4-a716-446655440001".to_string(),
        matching_status: Some("example".to_string()),
        metadata: Some(Default::default()),
        outstanding_amount: Some(rust_decimal::Decimal::new(15000, 0)),
        purchase_order_id: Some("example".to_string()),
        updated_at: Some("2024-01-15T09:00:00Z".to_string()),
        vendor_id: "411e8400-e29b-41d4-a716-446655440001".to_string(),
    }
}
