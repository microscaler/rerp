
// User-owned controller for handler 'create_mailing_list'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_mailing_list::{ Request, Response };



#[handler(CreateMailingListController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
