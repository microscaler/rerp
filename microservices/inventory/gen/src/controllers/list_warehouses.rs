
// User-owned controller for handler 'list_warehouses'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::list_warehouses::{ Request, Response };


#[allow(unused_imports)]
use crate::handlers::types::Warehouse;



#[handler(ListWarehousesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        items: Some(vec![]),limit: Some(42),page: Some(42),total: Some(42),
    }
    
    
}
