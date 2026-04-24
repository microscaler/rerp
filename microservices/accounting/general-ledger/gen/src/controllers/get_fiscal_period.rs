
// User-owned controller for handler 'get_fiscal_period'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_fiscal_period::{ Request, Response };



#[handler(GetFiscalPeriodController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        company_id: "example".to_string(),created_at: Some("example".to_string()),end_date: "example".to_string(),id: "example".to_string(),is_locked: true,is_open: Some(true),month: 42,name: "example".to_string(),start_date: "example".to_string(),updated_at: Some("example".to_string()),year: 42,
    }
    
    
}
