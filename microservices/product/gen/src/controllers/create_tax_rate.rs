
// User-owned controller for handler 'create_tax_rate'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_tax_rate::{ Request, Response };



#[handler(CreateTaxRateController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
