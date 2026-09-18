
// User-owned controller for handler 'get_timesheet_line'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_timesheet_line::{ Request, Response };



#[handler(GetTimesheetLineController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
