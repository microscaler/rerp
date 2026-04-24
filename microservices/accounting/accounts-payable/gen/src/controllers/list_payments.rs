
// User-owned controller for handler 'list_payments'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::list_payments::{ Request, Response };


#[allow(unused_imports)]
use crate::handlers::types::ApPayment;



#[handler(ListPaymentsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    // Example response:
        // {
        //   "items": [
        //     {
        //       "company_id": "550e8400-e29b-41d4-a716-446655440000",
        //       "created_at": "2024-01-20T10:00:00Z",
        //       "currency_code": "USD",
        //       "exchange_rate": 1.0,
        //       "id": "a0060e8400-e29b-41d4-a716-446655440000",
        //       "payment_amount": 7500.0,
        //       "payment_date": "2024-01-20",
        //       "payment_method": "WIRE",
        //       "payment_number": "AP-PAY-2024-001",
        //       "status": "POSTED",
        //       "updated_at": "2024-01-20T10:00:00Z",
        //       "vendor_id": "411e8400-e29b-41d4-a716-446655440001"
        //     }
        //   ],
        //   "limit": 20,
        //   "page": 1,
        //   "total": 1
        // }
    match serde_json::from_str::<Response>(r###"{
  "items": [
    {
      "company_id": "550e8400-e29b-41d4-a716-446655440000",
      "created_at": "2024-01-20T10:00:00Z",
      "currency_code": "USD",
      "exchange_rate": 1.0,
      "id": "a0060e8400-e29b-41d4-a716-446655440000",
      "payment_amount": 7500.0,
      "payment_date": "2024-01-20",
      "payment_method": "WIRE",
      "payment_number": "AP-PAY-2024-001",
      "status": "POSTED",
      "updated_at": "2024-01-20T10:00:00Z",
      "vendor_id": "411e8400-e29b-41d4-a716-446655440001"
    }
  ],
  "limit": 20,
  "page": 1,
  "total": 1
}"###) {
        Ok(parsed) => return parsed,
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }
    
    Response {
        items: Some(vec![serde_json::from_value::<ApPayment>(serde_json::json!({"company_id":"550e8400-e29b-41d4-a716-446655440000","created_at":"2024-01-20T10:00:00Z","currency_code":"USD","exchange_rate":1.0,"id":"a0060e8400-e29b-41d4-a716-446655440000","payment_amount":7500.0,"payment_date":"2024-01-20","payment_method":"WIRE","payment_number":"AP-PAY-2024-001","status":"POSTED","updated_at":"2024-01-20T10:00:00Z","vendor_id":"411e8400-e29b-41d4-a716-446655440001"})).unwrap_or_default()]),limit: Some(20),page: Some(1),total: Some(1),
    }
    
    
}
