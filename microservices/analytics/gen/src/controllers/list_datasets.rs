
// User-owned controller for handler 'list_datasets'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::list_datasets::{ Request, Response };


#[allow(unused_imports)]
use crate::handlers::types::Dataset;



#[handler(ListDatasetsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        items: Some(vec![]),limit: Some(42),page: Some(42),total: Some(42),
    }
    
    
}
