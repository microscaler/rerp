
// User-owned controller for handler 'delete_financial_statement'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_financial_statement::{ Request, Response };



#[handler(DeleteFinancialStatementController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
