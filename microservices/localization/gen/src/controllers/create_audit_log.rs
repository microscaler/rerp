
// User-owned controller for handler 'create_audit_log'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_audit_log::{ Request, Response };



#[handler(CreateAuditLogController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
