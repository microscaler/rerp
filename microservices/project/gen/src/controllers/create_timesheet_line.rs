
// User-owned controller for handler 'create_timesheet_line'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_timesheet_line::{ Request, Response };



#[handler(CreateTimesheetLineController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
