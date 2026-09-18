
// User-owned controller for handler 'verify_mfa'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::verify_mfa::{ Request, Response };



#[handler(VerifyMfaController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
