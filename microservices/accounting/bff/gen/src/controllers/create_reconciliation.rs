// User-owned controller for handler 'create_reconciliation'.
use crate::handlers::create_reconciliation::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateReconciliationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "bank_account_id": "a00d0e8400-e29b-41d4-a716-446655440000",
    //   "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //   "created_at": "2024-01-31T10:00:00Z",
    //   "currency_code": "USD",
    //   "ending_balance": 0.0,
    //   "id": "a00f0e8400-e29b-41d4-a716-446655440000",
    //   "reconciliation_date": "2024-01-31",
    //   "starting_balance": 0.0,
    //   "statement_id": "a00e0e8400-e29b-41d4-a716-446655440000",
    //   "status": "PENDING",
    //   "updated_at": "2024-01-31T10:00:00Z"
    // }

    Response {
        bank_account_id: "a00d0e8400-e29b-41d4-a716-446655440000".to_string(),
        bank_balance: Some(3.14),
        book_balance: Some(3.14),
        company_id: Some("550e8400-e29b-41d4-a716-446655440000".to_string()),
        created_at: Some("2024-01-31T10:00:00Z".to_string()),
        created_by: Some("example".to_string()),
        currency_code: "USD".to_string(),
        difference: Some(3.14),
        id: "a00f0e8400-e29b-41d4-a716-446655440000".to_string(),
        metadata: Some(Default::default()),
        notes: Some("example".to_string()),
        outstanding_deposits_amount: Some(3.14),
        outstanding_deposits_count: Some(42),
        outstanding_withdrawals_amount: Some(3.14),
        outstanding_withdrawals_count: Some(42),
        reconciled_at: Some("example".to_string()),
        reconciled_by: Some("example".to_string()),
        reconciliation_date: "2024-01-31".to_string(),
        statement_id: "a00e0e8400-e29b-41d4-a716-446655440000".to_string(),
        status: "PENDING".to_string(),
        updated_at: Some("2024-01-31T10:00:00Z".to_string()),
        updated_by: Some("example".to_string()),
    }
}
