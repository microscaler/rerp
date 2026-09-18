
// User-owned controller for handler 'delete_data_record'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::delete_data_record::{ Request, Response };



#[handler(DeleteDataRecordController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
