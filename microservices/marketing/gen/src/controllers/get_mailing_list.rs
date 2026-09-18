
// User-owned controller for handler 'get_mailing_list'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_mailing_list::{ Request, Response };



#[handler(GetMailingListController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
