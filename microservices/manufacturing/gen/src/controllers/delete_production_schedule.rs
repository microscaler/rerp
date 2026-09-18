
// User-owned controller for handler 'delete_production_schedule'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_production_schedule::{ Request, Response };



#[handler(DeleteProductionScheduleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
