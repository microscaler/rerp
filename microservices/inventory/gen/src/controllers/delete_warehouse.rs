
// User-owned controller for handler 'delete_warehouse'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_warehouse::{ Request, Response };



#[handler(DeleteWarehouseController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
