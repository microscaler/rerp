// User-owned controller for handler 'list_budget_lines'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_budget_gen::handlers::list_budget_lines::{Request, Response};

#[allow(unused_imports)]
use rerp_accounting_budget_gen::handlers::types::BudgetLine;

#[handler(ListBudgetLinesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "account_id": "a0070e8400-e29b-41d4-a716-446655440000",
    //       "budget_amount": rust_decimal::Decimal::new(100000, 0),
    //       "budget_id": "a0140e8400-e29b-41d4-a716-446655440000",
    //       "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //       "created_at": "2024-01-15T10:00:00Z",
    //       "currency_code": "USD",
    //       "id": "a0150e8400-e29b-41d4-a716-446655440000",
    //       "period_id": "a0160e8400-e29b-41d4-a716-446655440000",
    //       "updated_at": "2024-01-15T10:00:00Z"
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 1
    // }

    Response {
        items: Some(vec![serde_json::from_value::<BudgetLine>(serde_json::json!({"account_id":"a0070e8400-e29b-41d4-a716-446655440000","budget_amount":100000.0,"budget_id":"a0140e8400-e29b-41d4-a716-446655440000","company_id":"550e8400-e29b-41d4-a716-446655440000","created_at":"2024-01-15T10:00:00Z","currency_code":"USD","id":"a0150e8400-e29b-41d4-a716-446655440000","period_id":"a0160e8400-e29b-41d4-a716-446655440000","updated_at":"2024-01-15T10:00:00Z"})).unwrap_or_default()]),limit: Some(20),page: Some(1),total: Some(1),
    }
}
