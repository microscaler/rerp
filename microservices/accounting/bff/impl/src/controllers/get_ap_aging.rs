// User-owned controller for handler 'get_ap_aging'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::get_ap_aging::{Request, Response};

#[handler(GetApAgingController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "aging_date": "2024-01-31",
    //   "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //   "created_at": "2024-01-31T10:00:00Z",
    //   "currency_code": "USD",
    //   "current": rust_decimal::Decimal::new(7500, 0),
    //   "days_31_60": rust_decimal::Decimal::new(0, 0),
    //   "days_61_90": rust_decimal::Decimal::new(0, 0),
    //   "days_91_120": rust_decimal::Decimal::new(0, 0),
    //   "id": "a00b0e8400-e29b-41d4-a716-446655440000",
    //   "over_120": rust_decimal::Decimal::new(0, 0),
    //   "total_outstanding": rust_decimal::Decimal::new(7500, 0),
    //   "vendor_id": "411e8400-e29b-41d4-a716-446655440001"
    // }

    Response {
        aging_date: "2024-01-31".to_string(),
        company_id: Some("550e8400-e29b-41d4-a716-446655440000".to_string()),
        created_at: Some("2024-01-31T10:00:00Z".to_string()),
        currency_code: "USD".to_string(),
        current: Some(rust_decimal::Decimal::new(7500, 0)),
        days_31_60: Some(rust_decimal::Decimal::new(0, 0)),
        days_61_90: Some(rust_decimal::Decimal::new(0, 0)),
        days_91_120: Some(rust_decimal::Decimal::new(0, 0)),
        id: "a00b0e8400-e29b-41d4-a716-446655440000".to_string(),
        over_120: Some(rust_decimal::Decimal::new(0, 0)),
        total_outstanding: Some(rust_decimal::Decimal::new(7500, 0)),
        updated_at: Some("example".to_string()),
        vendor_id: "411e8400-e29b-41d4-a716-446655440001".to_string(),
    }
}
