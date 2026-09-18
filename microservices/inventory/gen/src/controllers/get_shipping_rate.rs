
// User-owned controller for handler 'get_shipping_rate'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_shipping_rate::{ Request, Response };



#[handler(GetShippingRateController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
