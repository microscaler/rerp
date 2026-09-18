
// User-owned controller for handler 'get_capacity_plan'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_capacity_plan::{ Request, Response };



#[handler(GetCapacityPlanController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
