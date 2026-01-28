// User-owned controller for handler 'list_budgets'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::list_budgets::{Request, Response};

#[allow(unused_imports)]
use rerp_accounting_bff_gen::handlers::types::Budget;

#[handler(ListBudgetsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "budget_name": "2024 Annual Budget",
    //       "budget_type": "ANNUAL",
    //       "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //       "created_at": "2024-01-15T10:00:00Z",
    //       "currency_code": "USD",
    //       "fiscal_year": 2024,
    //       "id": "a0140e8400-e29b-41d4-a716-446655440000",
    //       "status": "APPROVED",
    //       "total_budget_amount": rust_decimal::Decimal::new(10000000, 0),
    //       "updated_at": "2024-01-15T10:00:00Z"
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 1
    // }

    Response {
        items: Some(vec![serde_json::from_value::<Budget>(serde_json::json!({"budget_name":"2024 Annual Budget","budget_type":"ANNUAL","company_id":"550e8400-e29b-41d4-a716-446655440000","created_at":"2024-01-15T10:00:00Z","currency_code":"USD","fiscal_year":2024,"id":"a0140e8400-e29b-41d4-a716-446655440000","status":"APPROVED","total_budget_amount":10000000.0,"updated_at":"2024-01-15T10:00:00Z"})).unwrap_or_default()]),limit: Some(20),page: Some(1),total: Some(1),
    }
}
