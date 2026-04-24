
// User-owned controller for handler 'delete_fiscal_period'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_fiscal_period::{ Request, Response };



#[handler(DeleteFiscalPeriodController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        code: "example".to_string(),details: Some(Default::default()),message: "example".to_string(),
    }
    
    
}
