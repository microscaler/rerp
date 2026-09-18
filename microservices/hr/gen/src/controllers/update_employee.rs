
// User-owned controller for handler 'update_employee'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::update_employee::{ Request, Response };



#[handler(UpdateEmployeeController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
