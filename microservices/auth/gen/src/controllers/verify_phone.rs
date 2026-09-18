
// User-owned controller for handler 'verify_phone'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::verify_phone::{ Request, Response };



#[handler(VerifyPhoneController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
