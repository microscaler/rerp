
// User-owned controller for handler 'delete_dashboard'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_dashboard::{ Request, Response };



#[handler(DeleteDashboardController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
