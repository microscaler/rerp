
// User-owned controller for handler 'delete_account'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_account::{ Request, Response };



#[handler(DeleteAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        code: "example".to_string(),details: Some(Default::default()),message: "example".to_string(),
    }
    
    
}
