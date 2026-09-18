
// User-owned controller for handler 'delete_appraisal'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_appraisal::{ Request, Response };



#[handler(DeleteAppraisalController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
