
// User-owned controller for handler 'delete_bank_account'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_bank_account::{ Request, Response };



#[handler(DeleteBankAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
