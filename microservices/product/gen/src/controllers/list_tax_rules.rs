
// User-owned controller for handler 'list_tax_rules'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::list_tax_rules::{ Request, Response };


#[allow(unused_imports)]
use crate::handlers::types::TaxRule;



#[handler(ListTaxRulesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        items: Some(vec![]),limit: Some(42),page: Some(42),total: Some(42),
    }
    
    
}
