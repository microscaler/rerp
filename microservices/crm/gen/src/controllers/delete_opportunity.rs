
// User-owned controller for handler 'delete_opportunity'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_opportunity::{ Request, Response };



#[handler(DeleteOpportunityController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
