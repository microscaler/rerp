
// User-owned controller for handler 'revoke_permission'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::revoke_permission::{ Request, Response };



#[handler(RevokePermissionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
