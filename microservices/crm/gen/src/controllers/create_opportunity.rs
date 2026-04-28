
// User-owned controller for handler 'create_opportunity'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_opportunity::{ Request, Response };



#[handler(CreateOpportunityController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
