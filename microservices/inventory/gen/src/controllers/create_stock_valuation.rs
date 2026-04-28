
// User-owned controller for handler 'create_stock_valuation'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_stock_valuation::{ Request, Response };



#[handler(CreateStockValuationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
