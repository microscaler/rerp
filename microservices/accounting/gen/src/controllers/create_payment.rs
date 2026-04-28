
// User-owned controller for handler 'create_payment'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::create_payment::{ Request, Response };



#[handler(CreatePaymentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    // Example response:
        // {
        //   "company_id": "550e8400-e29b-41d4-a716-446655440000",
        //   "created_at": "2024-01-20T10:00:00Z",
        //   "currency_code": "USD",
        //   "customer_id": "111e8400-e29b-41d4-a716-446655440001",
        //   "exchange_rate": 1.0,
        //   "id": "a0040e8400-e29b-41d4-a716-446655440000",
        //   "payment_amount": 5500.0,
        //   "payment_date": "2024-01-20",
        //   "payment_method": "WIRE",
        //   "payment_number": "AR-PAY-2024-001",
        //   "status": "DRAFT",
        //   "updated_at": "2024-01-20T10:00:00Z"
        // }
    match serde_json::from_str::<Response>(r###"{
  "company_id": "550e8400-e29b-41d4-a716-446655440000",
  "created_at": "2024-01-20T10:00:00Z",
  "currency_code": "USD",
  "customer_id": "111e8400-e29b-41d4-a716-446655440001",
  "exchange_rate": 1.0,
  "id": "a0040e8400-e29b-41d4-a716-446655440000",
  "payment_amount": 5500.0,
  "payment_date": "2024-01-20",
  "payment_method": "WIRE",
  "payment_number": "AR-PAY-2024-001",
  "status": "DRAFT",
  "updated_at": "2024-01-20T10:00:00Z"
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
