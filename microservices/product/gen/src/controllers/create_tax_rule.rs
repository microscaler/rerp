
// User-owned controller for handler 'create_tax_rule'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_tax_rule::{ Request, Response };



#[handler(CreateTaxRuleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
