
// User-owned controller for handler 'delete_journal_entry'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_journal_entry::{ Request, Response };



#[handler(DeleteJournalEntryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        code: "example".to_string(),details: Some(Default::default()),message: "example".to_string(),
    }
    
    
}
