// User-owned controller for handler 'list_journal_entrys'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_general_ledger_gen::handlers::list_journal_entrys::{Request, Response};

#[allow(unused_imports)]
use rerp_accounting_general_ledger_gen::handlers::types::JournalEntry;

#[handler(ListJournalEntrysController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //       "created_at": "2024-01-15T09:00:00Z",
    //       "currency_code": "USD",
    //       "description": "Monthly accrual entry",
    //       "entry_date": "2024-01-15",
    //       "entry_number": "JE-2024-001",
    //       "exchange_rate": rust_decimal::Decimal::new(1, 0),
    //       "id": "a0080e8400-e29b-41d4-a716-446655440000",
    //       "posted_at": "2024-01-15T10:00:00Z",
    //       "source_type": "MANUAL",
    //       "status": "POSTED",
    //       "total_credit": rust_decimal::Decimal::new(5000, 0),
    //       "total_debit": rust_decimal::Decimal::new(5000, 0),
    //       "updated_at": "2024-01-15T10:00:00Z"
    //     },
    //     {
    //       "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //       "created_at": "2024-01-16T10:00:00Z",
    //       "currency_code": "USD",
    //       "description": "Invoice posting entry",
    //       "entry_date": "2024-01-16",
    //       "entry_number": "JE-2024-002",
    //       "exchange_rate": rust_decimal::Decimal::new(1, 0),
    //       "id": "a0081e8400-e29b-41d4-a716-446655440001",
    //       "posted_at": "2024-01-16T11:00:00Z",
    //       "source_id": "a0010e8400-e29b-41d4-a716-446655440000",
    //       "source_type": "INVOICE",
    //       "status": "POSTED",
    //       "total_credit": rust_decimal::Decimal::new(1200, 0),
    //       "total_debit": rust_decimal::Decimal::new(1200, 0),
    //       "updated_at": "2024-01-16T11:00:00Z"
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 2
    // }

    Response {
        items: Some(vec![serde_json::from_value::<JournalEntry>(serde_json::json!({"company_id":"550e8400-e29b-41d4-a716-446655440000","created_at":"2024-01-15T09:00:00Z","currency_code":"USD","description":"Monthly accrual entry","entry_date":"2024-01-15","entry_number":"JE-2024-001","exchange_rate":1.0,"id":"a0080e8400-e29b-41d4-a716-446655440000","posted_at":"2024-01-15T10:00:00Z","source_type":"MANUAL","status":"POSTED","total_credit":5000.0,"total_debit":5000.0,"updated_at":"2024-01-15T10:00:00Z"})).unwrap_or_default(), serde_json::from_value::<JournalEntry>(serde_json::json!({"company_id":"550e8400-e29b-41d4-a716-446655440000","created_at":"2024-01-16T10:00:00Z","currency_code":"USD","description":"Invoice posting entry","entry_date":"2024-01-16","entry_number":"JE-2024-002","exchange_rate":1.0,"id":"a0081e8400-e29b-41d4-a716-446655440001","posted_at":"2024-01-16T11:00:00Z","source_id":"a0010e8400-e29b-41d4-a716-446655440000","source_type":"INVOICE","status":"POSTED","total_credit":1200.0,"total_debit":1200.0,"updated_at":"2024-01-16T11:00:00Z"})).unwrap_or_default()]),limit: Some(20),page: Some(1),total: Some(2),
    }
}
