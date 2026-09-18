// User-owned controller for handler 'create_group_reporting_pack'.

use crate::handlers::create_group_reporting_pack::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateGroupReportingPackController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        artifact_uri: "example".to_string(),
        id: "example".to_string(),
        run_id: "example".to_string(),
        status: "example".to_string(),
    }
}
