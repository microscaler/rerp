
// User-owned controller for handler 'delete_order_line'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_order_line::{ Request, Response };



#[handler(DeleteOrderLineController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
