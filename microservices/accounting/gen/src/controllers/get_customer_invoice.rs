
// User-owned controller for handler 'get_customer_invoice'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::get_customer_invoice::{ Request, Response };



#[handler(GetCustomerInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    // Example response:
        // {
        //   "aging_bucket": "CURRENT",
        //   "company_id": "550e8400-e29b-41d4-a716-446655440000",
        //   "created_at": "2024-01-15T09:00:00Z",
        //   "currency_code": "USD",
        //   "customer_id": "111e8400-e29b-41d4-a716-446655440001",
        //   "id": "a0030e8400-e29b-41d4-a716-446655440000",
        //   "invoice_id": "a0010e8400-e29b-41d4-a716-446655440000",
        //   "original_amount": 11000.0,
        //   "outstanding_amount": 11000.0,
        //   "status": "OUTSTANDING",
        //   "updated_at": "2024-01-15T09:00:00Z"
        // }
    match serde_json::from_str::<Response>(r###"{
  "aging_bucket": "CURRENT",
  "company_id": "550e8400-e29b-41d4-a716-446655440000",
  "created_at": "2024-01-15T09:00:00Z",
  "currency_code": "USD",
  "customer_id": "111e8400-e29b-41d4-a716-446655440001",
  "id": "a0030e8400-e29b-41d4-a716-446655440000",
  "invoice_id": "a0010e8400-e29b-41d4-a716-446655440000",
  "original_amount": 11000.0,
  "outstanding_amount": 11000.0,
  "status": "OUTSTANDING",
  "updated_at": "2024-01-15T09:00:00Z"
}"###) {
        Ok(parsed) => return parsed,
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }
    
    Response {
        
    }
    
    
}
