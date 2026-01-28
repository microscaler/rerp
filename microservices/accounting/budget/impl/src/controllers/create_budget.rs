// User-owned controller for handler 'create_budget'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_budget_gen::handlers::create_budget::{Request, Response};

#[handler(CreateBudgetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "budget_name": "2024 Annual Budget",
    //   "budget_type": "ANNUAL",
    //   "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //   "created_at": "2024-01-15T10:00:00Z",
    //   "currency_code": "USD",
    //   "fiscal_year": 2024,
    //   "id": "a0140e8400-e29b-41d4-a716-446655440000",
    //   "status": "DRAFT",
    //   "total_budget_amount": rust_decimal::Decimal::new(0, 0),
    //   "updated_at": "2024-01-15T10:00:00Z"
    // }

    Response {
        approval_status: Some("example".to_string()),
        approved_at: Some("example".to_string()),
        approved_by: Some("example".to_string()),
        budget_number: "example".to_string(),
        company_id: Some("550e8400-e29b-41d4-a716-446655440000".to_string()),
        created_at: Some("2024-01-15T10:00:00Z".to_string()),
        created_by: Some("example".to_string()),
        currency_code: "USD".to_string(),
        current_version_id: Some("example".to_string()),
        description: Some("example".to_string()),
        fiscal_year: 2024,
        id: "a0140e8400-e29b-41d4-a716-446655440000".to_string(),
        metadata: Some(Default::default()),
        name: "example".to_string(),
        period_end: "example".to_string(),
        period_start: "example".to_string(),
        status: "DRAFT".to_string(),
        total_actual_amount: Some(rust_decimal::Decimal::new(314, 2)),
        total_budget_amount: Some(rust_decimal::Decimal::new(0, 0)),
        total_variance: Some(rust_decimal::Decimal::new(314, 2)),
        updated_at: Some("2024-01-15T10:00:00Z".to_string()),
        updated_by: Some("example".to_string()),
    }
}
