
// User-owned controller for handler 'get_holiday_calendar'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_holiday_calendar::{ Request, Response };



#[handler(GetHolidayCalendarController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
