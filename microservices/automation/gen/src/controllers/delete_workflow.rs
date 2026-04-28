
// User-owned controller for handler 'delete_workflow'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_workflow::{ Request, Response };



#[handler(DeleteWorkflowController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
