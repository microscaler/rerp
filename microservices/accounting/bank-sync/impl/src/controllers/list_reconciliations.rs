// User-owned controller for handler 'list_reconciliations'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bank_sync_gen::handlers::list_reconciliations::{Request, Response};

#[allow(unused_imports)]
use rerp_accounting_bank_sync_gen::handlers::types::Reconciliation;

#[handler(ListReconciliationsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "bank_account_id": "a00d0e8400-e29b-41d4-a716-446655440000",
    //       "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //       "created_at": "2024-01-31T10:00:00Z",
    //       "currency_code": "USD",
    //       "ending_balance": 50000.0,
    //       "id": "a00f0e8400-e29b-41d4-a716-446655440000",
    //       "reconciliation_date": "2024-01-31",
    //       "starting_balance": 45000.0,
    //       "statement_id": "a00e0e8400-e29b-41d4-a716-446655440000",
    //       "status": "COMPLETED",
    //       "updated_at": "2024-01-31T10:00:00Z"
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 1
    // }

    Response {
        items: Some(vec![serde_json::from_value::<Reconciliation>(serde_json::json!({"bank_account_id":"a00d0e8400-e29b-41d4-a716-446655440000","company_id":"550e8400-e29b-41d4-a716-446655440000","created_at":"2024-01-31T10:00:00Z","currency_code":"USD","ending_balance":50000.0,"id":"a00f0e8400-e29b-41d4-a716-446655440000","reconciliation_date":"2024-01-31","starting_balance":45000.0,"statement_id":"a00e0e8400-e29b-41d4-a716-446655440000","status":"COMPLETED","updated_at":"2024-01-31T10:00:00Z"})).unwrap_or_default()]),limit: Some(20),page: Some(1),total: Some(1),
    }
}
