// User-owned controller for handler 'list_edi_errors'.

use crate::handlers::list_edi_errors::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListEdiErrorsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
