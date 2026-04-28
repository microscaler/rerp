
// User-owned controller for handler 'get_leave_request'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_leave_request::{ Request, Response };



#[handler(GetLeaveRequestController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
