
// User-owned controller for handler 'list_availabilitys'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::list_availabilitys::{ Request, Response };


#[allow(unused_imports)]
use crate::handlers::types::Availability;



#[handler(ListAvailabilitysController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        items: Some(vec![]),limit: Some(42),page: Some(42),total: Some(42),
    }
    
    
}
