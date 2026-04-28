
// User-owned controller for handler 'get_payment_method'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_payment_method::{ Request, Response };



#[handler(GetPaymentMethodController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
