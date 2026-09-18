
// User-owned controller for handler 'delete_payment_method'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_payment_method::{ Request, Response };



#[handler(DeletePaymentMethodController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
