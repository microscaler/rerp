
// User-owned controller for handler 'delete_ap_aging'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_ap_aging::{ Request, Response };



#[handler(DeleteApAgingController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
