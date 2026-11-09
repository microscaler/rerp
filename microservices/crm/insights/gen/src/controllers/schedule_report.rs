// User-owned controller for handler 'schedule_report'.

use crate::handlers::schedule_report::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ScheduleReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "frequency": "Frequency",
    //   "id": "aa70dbe4-37c6-567e-b827-49d9e6bcd18c",
    //   "is_active": true,
    //   "last_sent": "2026-11-05T16:00:00Z",
    //   "recipient_ids": [
    //     "74ac0043-87c0-5717-854d-594f606bbda7"
    //   ],
    //   "report_id": "00daadeb-6175-5df5-b365-7a76ec2eaa99"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "frequency": "Frequency",
  "id": "aa70dbe4-37c6-567e-b827-49d9e6bcd18c",
  "is_active": true,
  "last_sent": "2026-11-05T16:00:00Z",
  "recipient_ids": [
    "74ac0043-87c0-5717-854d-594f606bbda7"
  ],
  "report_id": "00daadeb-6175-5df5-b365-7a76ec2eaa99"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        frequency: "Frequency".to_string(),
        id: Some("aa70dbe4-37c6-567e-b827-49d9e6bcd18c".to_string()),
        is_active: true,
        last_sent: "2026-11-05T16:00:00Z".to_string(),
        recipient_ids: vec!["74ac0043-87c0-5717-854d-594f606bbda7".to_string()],
        report_id: "00daadeb-6175-5df5-b365-7a76ec2eaa99".to_string(),
    })
}
