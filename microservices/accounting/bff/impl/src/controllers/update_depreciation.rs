// User-owned controller for handler 'update_depreciation'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::update_depreciation::{Request, Response};

#[handler(UpdateDepreciationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "accumulated_depreciation": rust_decimal::Decimal::new(55555556, 2),
    //   "asset_id": "a0100e8400-e29b-41d4-a716-446655440000",
    //   "book_value": rust_decimal::Decimal::new(444444444, 2),
    //   "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //   "created_at": "2024-01-31T10:00:00Z",
    //   "currency_code": "USD",
    //   "depreciation_amount": rust_decimal::Decimal::new(14000, 0),
    //   "id": "a0120e8400-e29b-41d4-a716-446655440000",
    //   "period_end": "2024-01-31",
    //   "period_start": "2024-01-01",
    //   "status": "POSTED",
    //   "updated_at": "2024-01-31T11:00:00Z"
    // }

    Response {
        accumulated_depreciation: Some(rust_decimal::Decimal::new(55555556, 2)),
        asset_id: "a0100e8400-e29b-41d4-a716-446655440000".to_string(),
        book_value: Some(rust_decimal::Decimal::new(444444444, 2)),
        created_at: Some("2024-01-31T10:00:00Z".to_string()),
        currency_code: "USD".to_string(),
        depreciation_amount: rust_decimal::Decimal::new(14000, 0),
        id: "a0120e8400-e29b-41d4-a716-446655440000".to_string(),
        journal_entry_id: Some("example".to_string()),
        period_end: "2024-01-31".to_string(),
        period_start: "2024-01-01".to_string(),
        posted_at: Some("example".to_string()),
        posted_by: Some("example".to_string()),
        status: "POSTED".to_string(),
        updated_at: Some("2024-01-31T11:00:00Z".to_string()),
    }
}
