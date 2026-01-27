// User-owned controller for handler 'list_accounts'.
use crate::handlers::list_accounts::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Account;

#[handler(ListAccountsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "account_type": "ASSET",
    //       "chart_of_account_id": "a00c0e8400-e29b-41d4-a716-446655440000",
    //       "code": "1010",
    //       "created_at": "2024-01-15T10:00:00Z",
    //       "currency_code": "USD",
    //       "description": "Primary operating cash account",
    //       "id": "a0070e8400-e29b-41d4-a716-446655440000",
    //       "is_active": true,
    //       "is_system_account": false,
    //       "name": "Cash - Operating Account",
    //       "normal_balance": "DEBIT",
    //       "updated_at": "2024-01-15T10:00:00Z"
    //     },
    //     {
    //       "account_type": "ASSET",
    //       "chart_of_account_id": "a00c0e8400-e29b-41d4-a716-446655440000",
    //       "code": "1200",
    //       "created_at": "2024-01-15T10:00:00Z",
    //       "currency_code": "USD",
    //       "description": "Trade receivables from customers",
    //       "id": "a0071e8400-e29b-41d4-a716-446655440001",
    //       "is_active": true,
    //       "is_system_account": false,
    //       "name": "Accounts Receivable",
    //       "normal_balance": "DEBIT",
    //       "updated_at": "2024-01-15T10:00:00Z"
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 2
    // }

    Response {
        items: Some(vec![serde_json::from_value::<Account>(serde_json::json!({"account_type":"ASSET","chart_of_account_id":"a00c0e8400-e29b-41d4-a716-446655440000","code":"1010","created_at":"2024-01-15T10:00:00Z","currency_code":"USD","description":"Primary operating cash account","id":"a0070e8400-e29b-41d4-a716-446655440000","is_active":true,"is_system_account":false,"name":"Cash - Operating Account","normal_balance":"DEBIT","updated_at":"2024-01-15T10:00:00Z"})).unwrap_or_default(), serde_json::from_value::<Account>(serde_json::json!({"account_type":"ASSET","chart_of_account_id":"a00c0e8400-e29b-41d4-a716-446655440000","code":"1200","created_at":"2024-01-15T10:00:00Z","currency_code":"USD","description":"Trade receivables from customers","id":"a0071e8400-e29b-41d4-a716-446655440001","is_active":true,"is_system_account":false,"name":"Accounts Receivable","normal_balance":"DEBIT","updated_at":"2024-01-15T10:00:00Z"})).unwrap_or_default()]),limit: Some(20),page: Some(1),total: Some(2),
    }
}
