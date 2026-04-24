
// User-owned controller for handler 'list_fiscal_periods'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::list_fiscal_periods::{ Request, Response };


#[allow(unused_imports)]
use crate::handlers::types::FiscalPeriod;



#[handler(ListFiscalPeriodsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        has_more: Some(true),items: vec![],limit: 42,page: 42,total: 42,
    }
    
    
}
