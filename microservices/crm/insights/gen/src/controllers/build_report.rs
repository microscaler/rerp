// User-owned controller for handler 'build_report'.

use crate::handlers::build_report::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(BuildReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    // Example response:
    // {
    //   "data": {},
    //   "report_id": "00daadeb-6175-5df5-b365-7a76ec2eaa99",
    //   "saved": false
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "data": {},
  "report_id": "00daadeb-6175-5df5-b365-7a76ec2eaa99",
  "saved": false
}"###,
    ) {
        Ok(parsed) => return ApiResponse::Ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    ApiResponse::Ok(Response {
        data: Some(
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null),
        ),
        report_id: Some("00daadeb-6175-5df5-b365-7a76ec2eaa99".to_string()),
        saved: Some(false),
    })
}
