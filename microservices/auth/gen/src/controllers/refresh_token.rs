
// User-owned controller for handler 'refresh_token'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::refresh_token::{ Request, Response };



#[handler(RefreshTokenController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
