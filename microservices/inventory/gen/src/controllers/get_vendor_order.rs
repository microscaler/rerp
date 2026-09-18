
// User-owned controller for handler 'get_vendor_order'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_vendor_order::{ Request, Response };



#[handler(GetVendorOrderController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
