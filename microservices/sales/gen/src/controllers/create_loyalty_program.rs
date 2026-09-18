
// User-owned controller for handler 'create_loyalty_program'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_loyalty_program::{ Request, Response };



#[handler(CreateLoyaltyProgramController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
