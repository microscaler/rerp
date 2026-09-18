
// User-owned controller for handler 'confirm_password_reset'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::confirm_password_reset::{ Request, Response };



#[handler(ConfirmPasswordResetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
