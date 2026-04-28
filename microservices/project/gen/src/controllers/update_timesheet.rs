
// User-owned controller for handler 'update_timesheet'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::update_timesheet::{ Request, Response };



#[handler(UpdateTimesheetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
