
// User-owned controller for handler 'get_sustainability_report'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_sustainability_report::{ Request, Response };



#[handler(GetSustainabilityReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
