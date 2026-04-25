
// User-owned controller for handler 'list_payments'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::list_payments::{ Request, Response };


#[allow(unused_imports)]
use crate::handlers::types::Payment;



#[handler(ListPaymentsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        has_more: Some(true),items: vec![],limit: 42,page: 42,total: 42,
    }
    
    
}
