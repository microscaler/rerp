
// User-owned controller for handler 'get_tax_rules_by_location'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_tax_rules_by_location::{ Request, Response };



#[handler(GetTaxRulesByLocationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
