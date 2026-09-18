
// User-owned controller for handler 'create_warehouse'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_warehouse::{ Request, Response };



#[handler(CreateWarehouseController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
