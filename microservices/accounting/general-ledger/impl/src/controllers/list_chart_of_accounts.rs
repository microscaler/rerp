// User-owned controller for handler 'list_chart_of_accounts'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_general_ledger_gen::handlers::list_chart_of_accounts::{Request, Response};

#[allow(unused_imports)]
use rerp_accounting_general_ledger_gen::handlers::types::ChartOfAccount;

#[handler(ListChartOfAccountsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "account_type": "ASSET",
    //       "code": "1",
    //       "created_at": "2024-01-15T10:00:00Z",
    //       "description": "Root asset account",
    //       "id": "a00c0e8400-e29b-41d4-a716-446655440000",
    //       "is_active": true,
    //       "level": 0,
    //       "name": "Assets",
    //       "updated_at": "2024-01-15T10:00:00Z"
    //     },
    //     {
    //       "account_type": "LIABILITY",
    //       "code": "2",
    //       "created_at": "2024-01-15T10:00:00Z",
    //       "description": "Root liability account",
    //       "id": "a00c1e8400-e29b-41d4-a716-446655440001",
    //       "is_active": true,
    //       "level": 0,
    //       "name": "Liabilities",
    //       "updated_at": "2024-01-15T10:00:00Z"
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 2
    // }

    Response {
        items: Some(vec![serde_json::from_value::<ChartOfAccount>(serde_json::json!({"account_type":"ASSET","code":"1","created_at":"2024-01-15T10:00:00Z","description":"Root asset account","id":"a00c0e8400-e29b-41d4-a716-446655440000","is_active":true,"level":0,"name":"Assets","updated_at":"2024-01-15T10:00:00Z"})).unwrap_or_default(), serde_json::from_value::<ChartOfAccount>(serde_json::json!({"account_type":"LIABILITY","code":"2","created_at":"2024-01-15T10:00:00Z","description":"Root liability account","id":"a00c1e8400-e29b-41d4-a716-446655440001","is_active":true,"level":0,"name":"Liabilities","updated_at":"2024-01-15T10:00:00Z"})).unwrap_or_default()]),limit: Some(20),page: Some(1),total: Some(2),
    }
}
