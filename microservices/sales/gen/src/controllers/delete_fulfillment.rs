
// User-owned controller for handler 'delete_fulfillment'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_fulfillment::{ Request, Response };



#[handler(DeleteFulfillmentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
