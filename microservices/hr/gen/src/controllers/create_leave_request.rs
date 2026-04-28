
// User-owned controller for handler 'create_leave_request'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_leave_request::{ Request, Response };



#[handler(CreateLeaveRequestController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
