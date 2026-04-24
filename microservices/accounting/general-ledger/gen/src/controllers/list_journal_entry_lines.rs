
// User-owned controller for handler 'list_journal_entry_lines'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::list_journal_entry_lines::{ Request, Response };


#[allow(unused_imports)]
use crate::handlers::types::JournalEntryLine;



#[handler(ListJournalEntryLinesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response(vec![])
    
    
}
