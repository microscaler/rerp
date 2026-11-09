// User-owned controller for handler 'get_marketing_revenue'.

use crate::handlers::get_marketing_revenue::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::NamedSeries;
#[allow(unused_imports)]
use crate::handlers::types::RevenueSourceRow;

#[handler(GetMarketingRevenueController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "attribution_window_days": 7,
    //   "currency": "EUR",
    //   "from_automation_messages": 41200,
    //   "from_campaigns": 27140,
    //   "series": [
    //     {
    //       "key": "automation",
    //       "label": "Automation emails",
    //       "points": [
    //         {
    //           "date": "2026-10-07",
    //           "value": 800
    //         },
    //         {
    //           "date": "2026-10-08",
    //           "value": 837
    //         },
    //         {
    //           "date": "2026-10-09",
    //           "value": 874
    //         }
    //       ]
    //     },
    //     {
    //       "key": "campaign",
    //       "label": "Campaigns",
    //       "points": [
    //         {
    //           "date": "2026-10-07",
    //           "value": 200
    //         },
    //         {
    //           "date": "2026-10-08",
    //           "value": 253
    //         },
    //         {
    //           "date": "2026-10-09",
    //           "value": 306
    //         }
    //       ]
    //     }
    //   ],
    //   "top_automation_messages": [
    //     {
    //       "conversions": 31,
    //       "id": "7689139b-3292-5a4d-82f6-5e6728e5eb2e",
    //       "kind": "AUTOMATION_MESSAGE",
    //       "name": "Day 7 · Upgrade offer",
    //       "revenue": "18600.00",
    //       "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
    //       "workflow_name": "Welcome series (7 days)"
    //     },
    //     {
    //       "conversions": 22,
    //       "id": "e9a85e92-da13-5077-90b2-942ce726419e",
    //       "kind": "AUTOMATION_MESSAGE",
    //       "name": "Renewal in 30 days",
    //       "revenue": "14100.00",
    //       "workflow_id": "1b370767-8ad9-50d6-9a2f-1417e9310e08",
    //       "workflow_name": "Renewal reminder · 30 days out"
    //     },
    //     {
    //       "conversions": 19,
    //       "id": "43187138-0f3a-56f8-94df-a8d0330ffb47",
    //       "kind": "AUTOMATION_MESSAGE",
    //       "name": "Day 5 · Customer stories",
    //       "revenue": "4800.00",
    //       "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
    //       "workflow_name": "Welcome series (7 days)"
    //     }
    //   ],
    //   "top_campaigns": [
    //     {
    //       "conversions": 24,
    //       "id": "7de39bcc-4bc6-551b-a967-7b1629b846be",
    //       "kind": "CAMPAIGN",
    //       "name": "October product update",
    //       "revenue": "12240.00"
    //     },
    //     {
    //       "conversions": 3,
    //       "id": "9995e2f3-21b8-5df2-a824-716f6159c4f5",
    //       "kind": "CAMPAIGN",
    //       "name": "Case study: Northwind",
    //       "revenue": "8200.00"
    //     },
    //     {
    //       "conversions": 18,
    //       "id": "cd847d34-2090-5e40-afc2-06c94055cc85",
    //       "kind": "CAMPAIGN",
    //       "name": "Webinar invite · pricing strategy",
    //       "revenue": "4900.00"
    //     }
    //   ],
    //   "total_conversions": 142,
    //   "total_revenue": "68340.00"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "attribution_window_days": 7,
  "currency": "EUR",
  "from_automation_messages": 41200,
  "from_campaigns": 27140,
  "series": [
    {
      "key": "automation",
      "label": "Automation emails",
      "points": [
        {
          "date": "2026-10-07",
          "value": 800
        },
        {
          "date": "2026-10-08",
          "value": 837
        },
        {
          "date": "2026-10-09",
          "value": 874
        }
      ]
    },
    {
      "key": "campaign",
      "label": "Campaigns",
      "points": [
        {
          "date": "2026-10-07",
          "value": 200
        },
        {
          "date": "2026-10-08",
          "value": 253
        },
        {
          "date": "2026-10-09",
          "value": 306
        }
      ]
    }
  ],
  "top_automation_messages": [
    {
      "conversions": 31,
      "id": "7689139b-3292-5a4d-82f6-5e6728e5eb2e",
      "kind": "AUTOMATION_MESSAGE",
      "name": "Day 7 · Upgrade offer",
      "revenue": "18600.00",
      "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
      "workflow_name": "Welcome series (7 days)"
    },
    {
      "conversions": 22,
      "id": "e9a85e92-da13-5077-90b2-942ce726419e",
      "kind": "AUTOMATION_MESSAGE",
      "name": "Renewal in 30 days",
      "revenue": "14100.00",
      "workflow_id": "1b370767-8ad9-50d6-9a2f-1417e9310e08",
      "workflow_name": "Renewal reminder · 30 days out"
    },
    {
      "conversions": 19,
      "id": "43187138-0f3a-56f8-94df-a8d0330ffb47",
      "kind": "AUTOMATION_MESSAGE",
      "name": "Day 5 · Customer stories",
      "revenue": "4800.00",
      "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
      "workflow_name": "Welcome series (7 days)"
    }
  ],
  "top_campaigns": [
    {
      "conversions": 24,
      "id": "7de39bcc-4bc6-551b-a967-7b1629b846be",
      "kind": "CAMPAIGN",
      "name": "October product update",
      "revenue": "12240.00"
    },
    {
      "conversions": 3,
      "id": "9995e2f3-21b8-5df2-a824-716f6159c4f5",
      "kind": "CAMPAIGN",
      "name": "Case study: Northwind",
      "revenue": "8200.00"
    },
    {
      "conversions": 18,
      "id": "cd847d34-2090-5e40-afc2-06c94055cc85",
      "kind": "CAMPAIGN",
      "name": "Webinar invite · pricing strategy",
      "revenue": "4900.00"
    }
  ],
  "total_conversions": 142,
  "total_revenue": "68340.00"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        attribution_window_days: 7,currency: "EUR".to_string(),from_automation_messages: 41200.0,from_campaigns: 27140.0,series: Some(vec![serde_json::from_value::<NamedSeries>(serde_json::json!({"key":"automation","label":"Automation emails","points":[{"date":"2026-10-07","value":800},{"date":"2026-10-08","value":837},{"date":"2026-10-09","value":874}]})).unwrap_or_default(), serde_json::from_value::<NamedSeries>(serde_json::json!({"key":"campaign","label":"Campaigns","points":[{"date":"2026-10-07","value":200},{"date":"2026-10-08","value":253},{"date":"2026-10-09","value":306}]})).unwrap_or_default()]),top_automation_messages: Some(vec![serde_json::from_value::<RevenueSourceRow>(serde_json::json!({"conversions":31,"id":"7689139b-3292-5a4d-82f6-5e6728e5eb2e","kind":"AUTOMATION_MESSAGE","name":"Day 7 · Upgrade offer","revenue":"18600.00","workflow_id":"22e49987-c27c-538a-9688-56d9ca17a7ec","workflow_name":"Welcome series (7 days)"})).unwrap_or_default(), serde_json::from_value::<RevenueSourceRow>(serde_json::json!({"conversions":22,"id":"e9a85e92-da13-5077-90b2-942ce726419e","kind":"AUTOMATION_MESSAGE","name":"Renewal in 30 days","revenue":"14100.00","workflow_id":"1b370767-8ad9-50d6-9a2f-1417e9310e08","workflow_name":"Renewal reminder · 30 days out"})).unwrap_or_default(), serde_json::from_value::<RevenueSourceRow>(serde_json::json!({"conversions":19,"id":"43187138-0f3a-56f8-94df-a8d0330ffb47","kind":"AUTOMATION_MESSAGE","name":"Day 5 · Customer stories","revenue":"4800.00","workflow_id":"22e49987-c27c-538a-9688-56d9ca17a7ec","workflow_name":"Welcome series (7 days)"})).unwrap_or_default()]),top_campaigns: Some(vec![serde_json::from_value::<RevenueSourceRow>(serde_json::json!({"conversions":24,"id":"7de39bcc-4bc6-551b-a967-7b1629b846be","kind":"CAMPAIGN","name":"October product update","revenue":"12240.00"})).unwrap_or_default(), serde_json::from_value::<RevenueSourceRow>(serde_json::json!({"conversions":3,"id":"9995e2f3-21b8-5df2-a824-716f6159c4f5","kind":"CAMPAIGN","name":"Case study: Northwind","revenue":"8200.00"})).unwrap_or_default(), serde_json::from_value::<RevenueSourceRow>(serde_json::json!({"conversions":18,"id":"cd847d34-2090-5e40-afc2-06c94055cc85","kind":"CAMPAIGN","name":"Webinar invite · pricing strategy","revenue":"4900.00"})).unwrap_or_default()]),total_conversions: Some(142),total_revenue: "68340.00".to_string(),
    })
}
