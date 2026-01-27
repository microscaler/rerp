// User-owned controller for handler 'create_bank_account'.
use crate::handlers::create_bank_account::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateBankAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "account_name": "Operating Account",
    //   "account_number": "CHASE-001",
    //   "account_type": "CHECKING",
    //   "balance": 0.0,
    //   "bank_name": "Chase Bank",
    //   "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //   "created_at": "2024-01-15T10:00:00Z",
    //   "currency_code": "USD",
    //   "id": "a00d0e8400-e29b-41d4-a716-446655440000",
    //   "is_active": true,
    //   "updated_at": "2024-01-15T10:00:00Z"
    // }

    Response {
        account_name: "Operating Account".to_string(),
        account_number: "CHASE-001".to_string(),
        account_type: "CHECKING".to_string(),
        bank_code: Some("example".to_string()),
        bank_name: Some("Chase Bank".to_string()),
        company_id: Some("550e8400-e29b-41d4-a716-446655440000".to_string()),
        created_at: Some("2024-01-15T10:00:00Z".to_string()),
        created_by: Some("example".to_string()),
        currency_code: "USD".to_string(),
        current_balance: Some(3.14),
        id: "a00d0e8400-e29b-41d4-a716-446655440000".to_string(),
        is_active: true,
        last_reconciled_at: Some("example".to_string()),
        last_synced_at: Some("example".to_string()),
        metadata: Some(Default::default()),
        reconciled_balance: Some(3.14),
        sync_credentials: Some("example".to_string()),
        sync_provider: Some("example".to_string()),
        updated_at: Some("2024-01-15T10:00:00Z".to_string()),
        updated_by: Some("example".to_string()),
    }
}
