// User-owned controller for handler 'update_ar_aging'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_accounts_receivable_gen::handlers::update_ar_aging::{Request, Response};

#[handler(UpdateArAgingController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "aging_date": "2024-01-31",
    //   "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //   "created_at": "2024-01-31T10:00:00Z",
    //   "currency_code": "USD",
    //   "current": 6000.0,
    //   "customer_id": "111e8400-e29b-41d4-a716-446655440001",
    //   "days_31_60": 0.0,
    //   "days_61_90": 0.0,
    //   "days_91_120": 0.0,
    //   "id": "a00a0e8400-e29b-41d4-a716-446655440000",
    //   "over_120": 0.0,
    //   "total_outstanding": 6000.0
    // }

    Response {
        aging_date: "2024-01-31".to_string(),
        company_id: Some("550e8400-e29b-41d4-a716-446655440000".to_string()),
        created_at: Some("2024-01-31T10:00:00Z".to_string()),
        currency_code: "USD".to_string(),
        current: Some(6000.0),
        customer_id: "111e8400-e29b-41d4-a716-446655440001".to_string(),
        days_31_60: Some(0.0),
        days_61_90: Some(0.0),
        days_91_120: Some(0.0),
        id: "a00a0e8400-e29b-41d4-a716-446655440000".to_string(),
        over_120: Some(0.0),
        total_outstanding: Some(6000.0),
        updated_at: Some("example".to_string()),
    }
}
