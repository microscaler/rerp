
// User-owned controller for handler 'get_stock_movement'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_stock_movement::{ Request, Response };



#[handler(GetStockMovementController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
