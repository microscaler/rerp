// User-owned controller for handler 'create_account'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::create_account::{Request, Response};

#[handler(CreateAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "account_type": "ASSET",
    //   "chart_of_account_id": "a00c0e8400-e29b-41d4-a716-446655440000",
    //   "code": "1010",
    //   "created_at": "2024-01-15T10:00:00Z",
    //   "currency_code": "USD",
    //   "description": "Primary operating cash account",
    //   "id": "a0070e8400-e29b-41d4-a716-446655440000",
    //   "is_active": true,
    //   "is_system_account": false,
    //   "name": "Cash - Operating Account",
    //   "normal_balance": "DEBIT",
    //   "updated_at": "2024-01-15T10:00:00Z"
    // }

    Response {
        account_type: "ASSET".to_string(),
        chart_of_account_id: "a00c0e8400-e29b-41d4-a716-446655440000".to_string(),
        code: "1010".to_string(),
        created_at: Some("2024-01-15T10:00:00Z".to_string()),
        currency_code: "USD".to_string(),
        description: Some("Primary operating cash account".to_string()),
        id: "a0070e8400-e29b-41d4-a716-446655440000".to_string(),
        is_active: true,
        is_system_account: false,
        metadata: Some(Default::default()),
        name: "Cash - Operating Account".to_string(),
        normal_balance: "DEBIT".to_string(),
        updated_at: Some("2024-01-15T10:00:00Z".to_string()),
    }
}
