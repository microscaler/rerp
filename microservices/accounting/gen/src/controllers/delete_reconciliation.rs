
// User-owned controller for handler 'delete_reconciliation'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_reconciliation::{ Request, Response };



#[handler(DeleteReconciliationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
