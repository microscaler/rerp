
// User-owned controller for handler 'update_stock_movement'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::update_stock_movement::{ Request, Response };



#[handler(UpdateStockMovementController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
