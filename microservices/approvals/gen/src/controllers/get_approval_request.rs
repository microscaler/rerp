
// User-owned controller for handler 'get_approval_request'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_approval_request::{ Request, Response };



#[handler(GetApprovalRequestController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
