
// User-owned controller for handler 'list_attendances'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::list_attendances::{ Request, Response };


#[allow(unused_imports)]
use crate::handlers::types::Attendance;



#[handler(ListAttendancesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        items: Some(vec![]),limit: Some(42),page: Some(42),total: Some(42),
    }
    
    
}
