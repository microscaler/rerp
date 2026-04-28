
// User-owned controller for handler 'update_job_posting'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::update_job_posting::{ Request, Response };



#[handler(UpdateJobPostingController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
