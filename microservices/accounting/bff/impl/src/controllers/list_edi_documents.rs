// User-owned controller for handler 'list_edi_documents'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::list_edi_documents::{Request, Response};

#[allow(unused_imports)]
use rerp_accounting_bff_gen::handlers::types::EdiDocument;

#[handler(ListEdiDocumentsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: Some(Default::default()),
        limit: Some(42),
        page: Some(42),
        total: Some(42),
    }
}
