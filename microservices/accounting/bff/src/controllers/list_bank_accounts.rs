// User-owned controller for handler 'list_bank_accounts'.
use crate::handlers::list_bank_accounts::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::BankAccount;

#[handler(ListBankAccountsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "account_name": "Operating Account",
    //       "account_number": "CHASE-001",
    //       "account_type": "CHECKING",
    //       "balance": 50000.0,
    //       "bank_name": "Chase Bank",
    //       "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //       "created_at": "2024-01-15T10:00:00Z",
    //       "currency_code": "USD",
    //       "id": "a00d0e8400-e29b-41d4-a716-446655440000",
    //       "is_active": true,
    //       "updated_at": "2024-01-15T10:00:00Z"
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 1
    // }

    Response {
        items: Some(vec![serde_json::from_value::<BankAccount>(serde_json::json!({"account_name":"Operating Account","account_number":"CHASE-001","account_type":"CHECKING","balance":50000.0,"bank_name":"Chase Bank","company_id":"550e8400-e29b-41d4-a716-446655440000","created_at":"2024-01-15T10:00:00Z","currency_code":"USD","id":"a00d0e8400-e29b-41d4-a716-446655440000","is_active":true,"updated_at":"2024-01-15T10:00:00Z"})).unwrap_or_default()]),limit: Some(20),page: Some(1),total: Some(1),
    }
}
