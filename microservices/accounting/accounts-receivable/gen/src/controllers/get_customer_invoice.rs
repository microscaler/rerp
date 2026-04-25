
// User-owned controller for handler 'get_customer_invoice'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_customer_invoice::{ Request, Response };



#[handler(GetCustomerInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
