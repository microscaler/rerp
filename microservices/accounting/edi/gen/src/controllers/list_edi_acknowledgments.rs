// User-owned controller for handler 'list_edi_acknowledgments'.

use crate::handlers::list_edi_acknowledgments::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListEdiAcknowledgmentsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
