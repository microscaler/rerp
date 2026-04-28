
// User-owned controller for handler 'create_esg_metric'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_esg_metric::{ Request, Response };



#[handler(CreateEsgMetricController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
