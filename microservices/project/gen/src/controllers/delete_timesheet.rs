
// User-owned controller for handler 'delete_timesheet'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_timesheet::{ Request, Response };



#[handler(DeleteTimesheetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
