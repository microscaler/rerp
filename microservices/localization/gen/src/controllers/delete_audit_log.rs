
// User-owned controller for handler 'delete_audit_log'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_audit_log::{ Request, Response };



#[handler(DeleteAuditLogController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
