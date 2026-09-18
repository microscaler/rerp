
// User-owned controller for handler 'delete_job_posting'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_job_posting::{ Request, Response };



#[handler(DeleteJobPostingController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
