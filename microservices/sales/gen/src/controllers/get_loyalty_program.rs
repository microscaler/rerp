
// User-owned controller for handler 'get_loyalty_program'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_loyalty_program::{ Request, Response };



#[handler(GetLoyaltyProgramController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
