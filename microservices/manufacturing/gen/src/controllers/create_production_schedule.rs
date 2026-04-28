
// User-owned controller for handler 'create_production_schedule'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_production_schedule::{ Request, Response };



#[handler(CreateProductionScheduleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
