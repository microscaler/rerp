
// User-owned controller for handler 'update_mailing_list'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::update_mailing_list::{ Request, Response };



#[handler(UpdateMailingListController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
