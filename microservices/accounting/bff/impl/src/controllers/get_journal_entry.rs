// User-owned controller for handler 'get_journal_entry'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::get_journal_entry::{Request, Response};

#[handler(GetJournalEntryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //   "created_at": "2024-01-15T09:00:00Z",
    //   "currency_code": "USD",
    //   "description": "Monthly accrual entry",
    //   "entry_date": "2024-01-15",
    //   "entry_number": "JE-2024-001",
    //   "exchange_rate": rust_decimal::Decimal::new(1, 0),
    //   "id": "a0080e8400-e29b-41d4-a716-446655440000",
    //   "posted_at": "2024-01-15T10:00:00Z",
    //   "source_type": "MANUAL",
    //   "status": "POSTED",
    //   "total_credit": rust_decimal::Decimal::new(5000, 0),
    //   "total_debit": rust_decimal::Decimal::new(5000, 0),
    //   "updated_at": "2024-01-15T10:00:00Z"
    // }

    Response {
        company_id: Some("550e8400-e29b-41d4-a716-446655440000".to_string()),
        created_at: Some("2024-01-15T09:00:00Z".to_string()),
        created_by: Some("example".to_string()),
        currency_code: "USD".to_string(),
        description: "Monthly accrual entry".to_string(),
        entry_date: "2024-01-15".to_string(),
        entry_number: "JE-2024-001".to_string(),
        exchange_rate: Some(rust_decimal::Decimal::new(1, 0)),
        fiscal_period_id: Some("example".to_string()),
        id: "a0080e8400-e29b-41d4-a716-446655440000".to_string(),
        metadata: Some(Default::default()),
        posted_at: Some("2024-01-15T10:00:00Z".to_string()),
        posted_by: Some("example".to_string()),
        reference_number: Some("example".to_string()),
        source_id: Some("example".to_string()),
        source_type: Some("MANUAL".to_string()),
        status: "POSTED".to_string(),
        total_credit: rust_decimal::Decimal::new(5000, 0),
        total_debit: rust_decimal::Decimal::new(5000, 0),
        updated_at: Some("2024-01-15T10:00:00Z".to_string()),
        updated_by: Some("example".to_string()),
    }
}
