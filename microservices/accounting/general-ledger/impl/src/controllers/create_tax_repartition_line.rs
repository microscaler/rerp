// Implementation stub for handler 'create_tax_repartition_line'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.
// To regenerate this stub, use: brrtrouter-gen generate-stubs --path create_tax_repartition_line --force

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_general_ledger_gen::handlers::create_tax_repartition_line::{
    Request, Response,
};

#[handler(CreateTaxRepartitionLineController)]
pub fn handle(req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let account_id = req.inner.account_id;// let percentage = req.inner.percentage;// let ratio = req.inner.ratio;// let repartition_type = req.inner.repartition_type;// let tag_ids = req.inner.tag_ids;// let tax_id = req.inner.tax_id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    Response {
        account_code: None,                      // TODO: Set from your business logic
        account_id: "example".to_string(),       // TODO: Set from your business logic
        account_name: None,                      // TODO: Set from your business logic
        created_at: None,                        // TODO: Set from your business logic
        id: "example".to_string(),               // TODO: Set from your business logic
        percentage: 3.14,                        // TODO: Set from your business logic
        ratio: 3.14,                             // TODO: Set from your business logic
        repartition_type: "example".to_string(), // TODO: Set from your business logic
        tag_ids: None,                           // TODO: Set from your business logic
        tax_id: "example".to_string(),           // TODO: Set from your business logic
        tax_name: None,                          // TODO: Set from your business logic
        updated_at: None,                        // TODO: Set from your business logic
    }
}
