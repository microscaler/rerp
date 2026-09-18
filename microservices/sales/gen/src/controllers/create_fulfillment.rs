
// User-owned controller for handler 'create_fulfillment'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_fulfillment::{ Request, Response };



#[handler(CreateFulfillmentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
