
// User-owned controller for handler 'create_payment_method'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_payment_method::{ Request, Response };



#[handler(CreatePaymentMethodController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
