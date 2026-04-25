
// User-owned controller for handler 'collections_summary'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::collections_summary::{ Request, Response };



#[handler(CollectionsSummaryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        by_type: Some(Default::default()),response_rate: Some(3.14),total_activities: Some(42),upcoming_follow_ups: Some(42),
    }
    
    
}
