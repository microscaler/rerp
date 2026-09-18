
// User-owned controller for handler 'create_repair_order'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_repair_order::{ Request, Response };



#[handler(CreateRepairOrderController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
