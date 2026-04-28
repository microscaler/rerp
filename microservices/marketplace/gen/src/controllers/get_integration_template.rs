
// User-owned controller for handler 'get_integration_template'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_integration_template::{ Request, Response };



#[handler(GetIntegrationTemplateController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
