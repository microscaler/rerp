
// User-owned controller for handler 'request_password_reset'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::request_password_reset::{ Request, Response };



#[handler(RequestPasswordResetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
