// User-owned controller for handler 'verify_sender'.

use crate::handlers::verify_sender::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DnsRecord;
#[allow(unused_imports)]
use crate::handlers::types::Provider;
#[allow(unused_imports)]
use crate::handlers::types::SenderStatus;

#[handler(VerifySenderController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "created_at": "2026-04-19T16:00:00Z",
    //   "dns_records": [
    //     {
    //       "name": "a1k3._domainkey",
    //       "purpose": "DKIM",
    //       "type": "CNAME",
    //       "value": "a1k3.dkim.amazonses.com",
    //       "verified": true
    //     },
    //     {
    //       "name": "@",
    //       "purpose": "SPF",
    //       "type": "TXT",
    //       "value": "v=spf1 include:amazonses.com -all",
    //       "verified": true
    //     },
    //     {
    //       "name": "_dmarc",
    //       "purpose": "DMARC",
    //       "type": "TXT",
    //       "value": "v=DMARC1; p=quarantine; rua=mailto:dmarc@acme.example",
    //       "verified": true
    //     }
    //   ],
    //   "domain": "acme.example",
    //   "from_address": "hello@acme.example",
    //   "from_name": "Acme Co",
    //   "id": "37371e40-5aeb-5ad2-a9e6-389a2d6f0ca2",
    //   "last_checked_at": "2026-11-05T15:56:00Z",
    //   "postal_address": "Acme Co, 12 Example Street, 10115 Berlin, Germany",
    //   "provider": "SES",
    //   "reply_to": "support@acme.example",
    //   "status": "VERIFIED"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "created_at": "2026-04-19T16:00:00Z",
  "dns_records": [
    {
      "name": "a1k3._domainkey",
      "purpose": "DKIM",
      "type": "CNAME",
      "value": "a1k3.dkim.amazonses.com",
      "verified": true
    },
    {
      "name": "@",
      "purpose": "SPF",
      "type": "TXT",
      "value": "v=spf1 include:amazonses.com -all",
      "verified": true
    },
    {
      "name": "_dmarc",
      "purpose": "DMARC",
      "type": "TXT",
      "value": "v=DMARC1; p=quarantine; rua=mailto:dmarc@acme.example",
      "verified": true
    }
  ],
  "domain": "acme.example",
  "from_address": "hello@acme.example",
  "from_name": "Acme Co",
  "id": "37371e40-5aeb-5ad2-a9e6-389a2d6f0ca2",
  "last_checked_at": "2026-11-05T15:56:00Z",
  "postal_address": "Acme Co, 12 Example Street, 10115 Berlin, Germany",
  "provider": "SES",
  "reply_to": "support@acme.example",
  "status": "VERIFIED"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        created_at: "2026-04-19T16:00:00Z".to_string(),dns_records: Some(vec![serde_json::from_value::<DnsRecord>(serde_json::json!({"name":"a1k3._domainkey","purpose":"DKIM","type":"CNAME","value":"a1k3.dkim.amazonses.com","verified":true})).unwrap_or_default(), serde_json::from_value::<DnsRecord>(serde_json::json!({"name":"@","purpose":"SPF","type":"TXT","value":"v=spf1 include:amazonses.com -all","verified":true})).unwrap_or_default(), serde_json::from_value::<DnsRecord>(serde_json::json!({"name":"_dmarc","purpose":"DMARC","type":"TXT","value":"v=DMARC1; p=quarantine; rua=mailto:dmarc@acme.example","verified":true})).unwrap_or_default()]),domain: "acme.example".to_string(),from_address: "hello@acme.example".to_string(),from_name: "Acme Co".to_string(),id: "37371e40-5aeb-5ad2-a9e6-389a2d6f0ca2".to_string(),last_checked_at: Some("2026-11-05T15:56:00Z".to_string()),postal_address: Some("Acme Co, 12 Example Street, 10115 Berlin, Germany".to_string()),provider: Some(serde_json::from_value::<Provider>(serde_json::Value::String("SES".to_string())).unwrap_or_default()),reply_to: Some("support@acme.example".to_string()),status: serde_json::from_value::<SenderStatus>(serde_json::Value::String("VERIFIED".to_string())).unwrap_or_default(),
    })
}
