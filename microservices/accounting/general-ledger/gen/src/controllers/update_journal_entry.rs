// User-owned controller for handler 'update_journal_entry'.

use crate::handlers::update_journal_entry::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateJournalEntryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        company_id: "example".to_string(),
        created_at: "example".to_string(),
        created_by: "example".to_string(),
        currency_code: "example".to_string(),
        description: "example".to_string(),
        entry_date: "example".to_string(),
        entry_number: "example".to_string(),
        exchange_rate: 3.14,
        fiscal_period_id: "example".to_string(),
        id: "example".to_string(),
        journal_id: "example".to_string(),
        posted_at: "example".to_string(),
        posted_by: Some("example".to_string()),
        reference_number: "example".to_string(),
        reversed_at: "example".to_string(),
        reversed_by: Some("example".to_string()),
        status: "example".to_string(),
        total_credit: 3.14,
        total_debit: 3.14,
        updated_at: Some("example".to_string()),
        updated_by: Some("example".to_string()),
    })
}
