
// User-owned controller for handler 'get_approval_workflow'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_approval_workflow::{ Request, Response };



#[handler(GetApprovalWorkflowController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
