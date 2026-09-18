
// User-owned controller for handler 'create_dashboard'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_dashboard::{ Request, Response };



#[handler(CreateDashboardController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
