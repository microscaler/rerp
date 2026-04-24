
// User-owned controller for handler 'get_journal_entry'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_journal_entry::{ Request, Response };



#[handler(GetJournalEntryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
