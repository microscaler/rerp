
// User-owned controller for handler 'list_tickets'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::list_tickets::{ Request, Response };


#[allow(unused_imports)]
use crate::handlers::types::Ticket;



#[handler(ListTicketsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        items: Some(vec![]),limit: Some(42),page: Some(42),total: Some(42),
    }
    
    
}
