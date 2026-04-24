
// User-owned controller for handler 'bulk_post_journal_entries'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::bulk_post_journal_entries::{ Request, Response };



#[handler(BulkPostJournalEntriesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        failed: 42,results: vec![],succeeded: 42,total: 42,
    }
    
    
}
