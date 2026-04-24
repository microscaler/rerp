
// User-owned controller for handler 'delete_bank_statement'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_bank_statement::{ Request, Response };



#[handler(DeleteBankStatementController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
