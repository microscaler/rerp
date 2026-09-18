
// User-owned controller for handler 'trigger_webhook'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::trigger_webhook::{ Request, Response };



#[handler(TriggerWebhookController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
