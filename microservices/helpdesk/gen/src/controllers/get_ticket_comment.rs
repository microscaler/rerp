
// User-owned controller for handler 'get_ticket_comment'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_ticket_comment::{ Request, Response };



#[handler(GetTicketCommentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
