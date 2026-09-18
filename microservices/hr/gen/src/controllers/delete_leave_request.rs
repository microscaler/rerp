
// User-owned controller for handler 'delete_leave_request'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_leave_request::{ Request, Response };



#[handler(DeleteLeaveRequestController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
