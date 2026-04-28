
// User-owned controller for handler 'list_approval_requests'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::list_approval_requests::{ Request, Response };


#[allow(unused_imports)]
use crate::handlers::types::ApprovalRequest;



#[handler(ListApprovalRequestsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        items: Some(vec![]),limit: Some(42),page: Some(42),total: Some(42),
    }
    
    
}
