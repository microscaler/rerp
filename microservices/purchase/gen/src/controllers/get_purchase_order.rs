
// User-owned controller for handler 'get_purchase_order'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_purchase_order::{ Request, Response };



#[handler(GetPurchaseOrderController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
