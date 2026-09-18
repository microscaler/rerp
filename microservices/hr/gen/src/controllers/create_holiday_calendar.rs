
// User-owned controller for handler 'create_holiday_calendar'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_holiday_calendar::{ Request, Response };



#[handler(CreateHolidayCalendarController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
