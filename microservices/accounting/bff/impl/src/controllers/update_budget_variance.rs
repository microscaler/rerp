// User-owned controller for handler 'update_budget_variance'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::update_budget_variance::{Request, Response};

#[handler(UpdateBudgetVarianceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "account_id": "a0070e8400-e29b-41d4-a716-446655440000",
    //   "actual_amount": rust_decimal::Decimal::new(98000, 0),
    //   "budget_amount": rust_decimal::Decimal::new(100000, 0),
    //   "budget_id": "a0140e8400-e29b-41d4-a716-446655440000",
    //   "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //   "created_at": "2024-01-31T10:00:00Z",
    //   "currency_code": "USD",
    //   "id": "a0170e8400-e29b-41d4-a716-446655440000",
    //   "period_id": "a0160e8400-e29b-41d4-a716-446655440000",
    //   "updated_at": "2024-01-31T11:00:00Z",
    //   "variance_amount": -2000.0,
    //   "variance_percent": -2.0
    // }

    Response {
        account_id: "a0070e8400-e29b-41d4-a716-446655440000".to_string(),
        actual_amount: Some(rust_decimal::Decimal::new(98000, 0)),
        budget_amount: Some(rust_decimal::Decimal::new(100000, 0)),
        budget_id: "a0140e8400-e29b-41d4-a716-446655440000".to_string(),
        created_at: Some("2024-01-31T10:00:00Z".to_string()),
        currency_code: "USD".to_string(),
        exceeds_threshold: Some(true),
        id: "a0170e8400-e29b-41d4-a716-446655440000".to_string(),
        is_favorable: Some(true),
        last_calculated_at: Some("example".to_string()),
        period_id: "a0160e8400-e29b-41d4-a716-446655440000".to_string(),
        updated_at: Some("2024-01-31T11:00:00Z".to_string()),
        variance: Some(rust_decimal::Decimal::new(314, 2)),
        variance_percent: Some(rust_decimal::Decimal::new(-2, 0)),
    }
}
