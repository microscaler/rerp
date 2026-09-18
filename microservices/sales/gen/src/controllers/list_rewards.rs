
// User-owned controller for handler 'list_rewards'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::list_rewards::{ Request, Response };


#[allow(unused_imports)]
use crate::handlers::types::Reward;



#[handler(ListRewardsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        items: Some(vec![]),limit: Some(42),page: Some(42),total: Some(42),
    }
    
    
}
