
// User-owned controller for handler 'get_points_transaction'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_points_transaction::{ Request, Response };



#[handler(GetPointsTransactionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
