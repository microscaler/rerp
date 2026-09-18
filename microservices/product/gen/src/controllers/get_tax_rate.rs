
// User-owned controller for handler 'get_tax_rate'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_tax_rate::{ Request, Response };



#[handler(GetTaxRateController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
