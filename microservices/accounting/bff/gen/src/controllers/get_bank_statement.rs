// User-owned controller for handler 'get_bank_statement'.
use crate::handlers::get_bank_statement::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetBankStatementController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "bank_account_id": "a00d0e8400-e29b-41d4-a716-446655440000",
    //   "closing_balance": 50000.0,
    //   "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //   "created_at": "2024-01-31T10:00:00Z",
    //   "currency_code": "USD",
    //   "id": "a00e0e8400-e29b-41d4-a716-446655440000",
    //   "opening_balance": 45000.0,
    //   "statement_date": "2024-01-31",
    //   "status": "RECONCILED",
    //   "updated_at": "2024-01-31T10:00:00Z"
    // }

    Response {
        bank_account_id: "a00d0e8400-e29b-41d4-a716-446655440000".to_string(),
        closing_balance: Some(rust_decimal::Decimal::new(500000, 1)),
        company_id: Some("550e8400-e29b-41d4-a716-446655440000".to_string()),
        created_at: Some("2024-01-31T10:00:00Z".to_string()),
        created_by: Some("example".to_string()),
        currency_code: "USD".to_string(),
        id: "a00e0e8400-e29b-41d4-a716-446655440000".to_string(),
        import_format: Some("example".to_string()),
        import_source: Some("example".to_string()),
        imported_at: Some("example".to_string()),
        imported_by: Some("example".to_string()),
        metadata: Some(Default::default()),
        opening_balance: Some(rust_decimal::Decimal::new(450000, 1)),
        period_end: Some("example".to_string()),
        period_start: Some("example".to_string()),
        reconciled_at: Some("example".to_string()),
        reconciled_by: Some("example".to_string()),
        statement_date: "2024-01-31".to_string(),
        statement_number: Some("example".to_string()),
        status: "RECONCILED".to_string(),
        total_credits: Some(rust_decimal::Decimal::new(12345, 2)),
        total_debits: Some(rust_decimal::Decimal::new(12345, 2)),
        transaction_count: Some(42),
        updated_at: Some("2024-01-31T10:00:00Z".to_string()),
        updated_by: Some("example".to_string()),
    }
}
