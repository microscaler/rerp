
// User-owned controller for handler 'get_skill_requirement'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_skill_requirement::{ Request, Response };



#[handler(GetSkillRequirementController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        
    }
    
    
}
