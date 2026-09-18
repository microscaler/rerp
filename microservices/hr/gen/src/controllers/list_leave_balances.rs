
// User-owned controller for handler 'list_leave_balances'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::list_leave_balances::{ Request, Response };


#[allow(unused_imports)]
use crate::handlers::types::LeaveBalance;



#[handler(ListLeaveBalancesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        items: Some(vec![]),limit: Some(42),page: Some(42),total: Some(42),
    }
    
    
}
