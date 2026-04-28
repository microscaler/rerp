
// User-owned controller for handler 'list_job_postings'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::list_job_postings::{ Request, Response };


#[allow(unused_imports)]
use crate::handlers::types::JobPosting;



#[handler(ListJobPostingsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        items: Some(vec![]),limit: Some(42),page: Some(42),total: Some(42),
    }
    
    
}
