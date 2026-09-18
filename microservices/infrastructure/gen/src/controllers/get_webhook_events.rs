
// User-owned controller for handler 'get_webhook_events'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_webhook_events::{ Request, Response };



#[handler(GetWebhookEventsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
