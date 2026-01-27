// User-owned controller for handler 'delete_customer_invoice'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_accounts_receivable_gen::handlers::delete_customer_invoice::{
    Request, Response,
};

#[handler(DeleteCustomerInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
