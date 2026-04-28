
// User-owned controller for handler 'delete_milestone'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_milestone::{ Request, Response };



#[handler(DeleteMilestoneController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
