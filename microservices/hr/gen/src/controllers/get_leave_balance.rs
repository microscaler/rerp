
// User-owned controller for handler 'get_leave_balance'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_leave_balance::{ Request, Response };



#[handler(GetLeaveBalanceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
