// User-owned controller for handler 'update_payment'.
use crate::handlers::update_payment::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdatePaymentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //   "created_at": "2024-01-20T10:00:00Z",
    //   "currency_code": "USD",
    //   "exchange_rate": 1.0,
    //   "id": "a0060e8400-e29b-41d4-a716-446655440000",
    //   "payment_amount": 7500.0,
    //   "payment_date": "2024-01-20",
    //   "payment_method": "WIRE",
    //   "payment_number": "AP-PAY-2024-001",
    //   "status": "POSTED",
    //   "updated_at": "2024-01-20T11:00:00Z",
    //   "vendor_id": "411e8400-e29b-41d4-a716-446655440001"
    // }

    Response {
        applied_amount: Some(rust_decimal::Decimal::new(12345, 2)),
        bank_account_id: Some("example".to_string()),
        company_id: Some("550e8400-e29b-41d4-a716-446655440000".to_string()),
        created_at: Some("2024-01-20T10:00:00Z".to_string()),
        created_by: Some("example".to_string()),
        currency_code: "USD".to_string(),
        id: "a0060e8400-e29b-41d4-a716-446655440000".to_string(),
        metadata: Some(Default::default()),
        notes: Some("example".to_string()),
        payment_amount: rust_decimal::Decimal::new(75000, 1),
        payment_date: "2024-01-20".to_string(),
        payment_method: "WIRE".to_string(),
        payment_number: "AP-PAY-2024-001".to_string(),
        payment_reference: Some("example".to_string()),
        status: "POSTED".to_string(),
        updated_at: Some("2024-01-20T11:00:00Z".to_string()),
        updated_by: Some("example".to_string()),
        vendor_id: "411e8400-e29b-41d4-a716-446655440001".to_string(),
    }
}
