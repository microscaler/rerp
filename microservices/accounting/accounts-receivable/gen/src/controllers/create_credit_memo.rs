
// User-owned controller for handler 'create_credit_memo'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_credit_memo::{ Request, Response };



#[handler(CreateCreditMemoController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        amount: 3.14,company_id: Some("example".to_string()),created_at: Some("example".to_string()),currency_code: "example".to_string(),customer_id: Some("example".to_string()),customer_invoice_id: "example".to_string(),description: Some("example".to_string()),id: "example".to_string(),reason: "example".to_string(),reference_id: Some("example".to_string()),remaining_amount: Some(3.14),status: Some("example".to_string()),updated_at: Some("example".to_string()),
    }
    
    
}
