// User-owned controller for handler 'list_budget_variances'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_budget_gen::handlers::list_budget_variances::{Request, Response};

#[allow(unused_imports)]
use rerp_accounting_budget_gen::handlers::types::BudgetVariance;

#[handler(ListBudgetVariancesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "account_id": "a0070e8400-e29b-41d4-a716-446655440000",
    //       "actual_amount": 95000.0,
    //       "budget_amount": 100000.0,
    //       "budget_id": "a0140e8400-e29b-41d4-a716-446655440000",
    //       "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //       "created_at": "2024-01-31T10:00:00Z",
    //       "currency_code": "USD",
    //       "id": "a0170e8400-e29b-41d4-a716-446655440000",
    //       "period_id": "a0160e8400-e29b-41d4-a716-446655440000",
    //       "updated_at": "2024-01-31T10:00:00Z",
    //       "variance_amount": -5000.0,
    //       "variance_percent": -5.0
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 1
    // }

    Response {
        items: Some(vec![serde_json::from_value::<BudgetVariance>(serde_json::json!({"account_id":"a0070e8400-e29b-41d4-a716-446655440000","actual_amount":95000.0,"budget_amount":100000.0,"budget_id":"a0140e8400-e29b-41d4-a716-446655440000","company_id":"550e8400-e29b-41d4-a716-446655440000","created_at":"2024-01-31T10:00:00Z","currency_code":"USD","id":"a0170e8400-e29b-41d4-a716-446655440000","period_id":"a0160e8400-e29b-41d4-a716-446655440000","updated_at":"2024-01-31T10:00:00Z","variance_amount":-5000.0,"variance_percent":-5.0})).unwrap_or_default()]),limit: Some(20),page: Some(1),total: Some(1),
    }
}
