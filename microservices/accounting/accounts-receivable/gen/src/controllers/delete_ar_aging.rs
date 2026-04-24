
// User-owned controller for handler 'delete_ar_aging'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_ar_aging::{ Request, Response };



#[handler(DeleteArAgingController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
