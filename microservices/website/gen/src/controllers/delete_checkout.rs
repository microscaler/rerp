
// User-owned controller for handler 'delete_checkout'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_checkout::{ Request, Response };



#[handler(DeleteCheckoutController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
