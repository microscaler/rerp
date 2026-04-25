
// User-owned controller for handler 'list_payment_applications'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::list_payment_applications::{ Request, Response };


#[allow(unused_imports)]
use crate::handlers::types::PaymentApplication;



#[handler(ListPaymentApplicationsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response(vec![])
    
    
}
