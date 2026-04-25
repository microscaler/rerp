
// User-owned controller for handler 'list_ar_agings'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::list_ar_agings::{ Request, Response };


#[allow(unused_imports)]
use crate::handlers::types::ArAging;



#[handler(ListArAgingsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        has_more: Some(true),items: vec![],limit: 42,page: 42,total: 42,
    }
    
    
}
