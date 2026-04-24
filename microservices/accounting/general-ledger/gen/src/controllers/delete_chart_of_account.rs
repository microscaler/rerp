
// User-owned controller for handler 'delete_chart_of_account'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_chart_of_account::{ Request, Response };



#[handler(DeleteChartOfAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        code: "example".to_string(),details: Some(Default::default()),message: "example".to_string(),
    }
    
    
}
