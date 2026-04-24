
// User-owned controller for handler 'create_journal_entry'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_journal_entry::{ Request, Response };



#[handler(CreateJournalEntryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        company_id: Some("example".to_string()),created_at: Some("example".to_string()),created_by: Some("example".to_string()),currency_code: "example".to_string(),description: "example".to_string(),entry_date: "example".to_string(),entry_number: "example".to_string(),exchange_rate: Some(3.14),fiscal_period_id: Some("example".to_string()),id: "example".to_string(),journal_id: Some("example".to_string()),posted_at: Some("example".to_string()),posted_by: Some("example".to_string()),reference_number: Some("example".to_string()),reversed_at: Some("example".to_string()),reversed_by: Some("example".to_string()),status: "example".to_string(),total_credit: 3.14,total_debit: 3.14,updated_at: Some("example".to_string()),updated_by: Some("example".to_string()),
    }
    
    
}
