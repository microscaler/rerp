
// User-owned controller for handler 'create_ticket_comment'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_ticket_comment::{ Request, Response };



#[handler(CreateTicketCommentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
