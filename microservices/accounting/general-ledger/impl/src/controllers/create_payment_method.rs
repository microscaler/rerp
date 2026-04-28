// Implementation stub for handler 'create_payment_method'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.
// To regenerate this stub, use: brrtrouter-gen generate-stubs --path create_payment_method --force

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_general_ledger_gen::handlers::create_payment_method::{Request, Response};

#[allow(unused_imports)]
use rerp_accounting_general_ledger_gen::handlers::types::PaymentMethodJournalMappingRequest;

#[handler(CreatePaymentMethodController)]
pub fn handle(req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let code = req.inner.code;// let company_id = req.inner.company_id;// let description = req.inner.description;// let is_active = req.inner.is_active;// let is_payable = req.inner.is_payable;// let is_receivable = req.inner.is_receivable;// let journal_mappings = req.inner.journal_mappings;// let name = req.inner.name;// let payment_method_type = req.inner.payment_method_type;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    Response {
        code: "example".to_string(),       // TODO: Set from your business logic
        company_id: "example".to_string(), // TODO: Set from your business logic
        created_at: None,                  // TODO: Set from your business logic
        description: None,                 // TODO: Set from your business logic
        id: "example".to_string(),         // TODO: Set from your business logic
        is_active: true,                   // TODO: Set from your business logic
        is_payable: None,                  // TODO: Set from your business logic
        is_receivable: None,               // TODO: Set from your business logic
        name: "example".to_string(),       // TODO: Set from your business logic
        payment_method_type: "example".to_string(), // TODO: Set from your business logic
        updated_at: None,                  // TODO: Set from your business logic
    }
}
