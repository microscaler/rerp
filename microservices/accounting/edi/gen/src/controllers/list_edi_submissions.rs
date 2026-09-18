// User-owned controller for handler 'list_edi_submissions'.

use crate::handlers::list_edi_submissions::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListEdiSubmissionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
