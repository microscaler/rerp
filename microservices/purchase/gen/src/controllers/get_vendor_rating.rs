
// User-owned controller for handler 'get_vendor_rating'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_vendor_rating::{ Request, Response };



#[handler(GetVendorRatingController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
