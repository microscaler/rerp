
// User-owned controller for handler 'delete_stock_movement'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_stock_movement::{ Request, Response };



#[handler(DeleteStockMovementController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
