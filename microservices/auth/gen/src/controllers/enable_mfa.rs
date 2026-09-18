
// User-owned controller for handler 'enable_mfa'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::enable_mfa::{ Request, Response };



#[handler(EnableMfaController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
