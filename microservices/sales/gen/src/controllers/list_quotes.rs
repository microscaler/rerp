
// User-owned controller for handler 'list_quotes'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::list_quotes::{ Request, Response };


#[allow(unused_imports)]
use crate::handlers::types::Quote;



#[handler(ListQuotesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        items: Some(vec![]),limit: Some(42),page: Some(42),total: Some(42),
    }
    
    
}
