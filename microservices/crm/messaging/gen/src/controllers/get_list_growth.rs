// User-owned controller for handler 'get_list_growth'.

use crate::handlers::get_list_growth::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetListGrowthController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        list_id: "example".to_string(),
        points: vec![],
    })
}
