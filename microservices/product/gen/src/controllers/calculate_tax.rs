
// User-owned controller for handler 'calculate_tax'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::calculate_tax::{ Request, Response };



#[handler(CalculateTaxController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
