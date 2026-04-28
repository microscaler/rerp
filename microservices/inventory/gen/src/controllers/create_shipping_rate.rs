
// User-owned controller for handler 'create_shipping_rate'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_shipping_rate::{ Request, Response };



#[handler(CreateShippingRateController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
