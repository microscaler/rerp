//! The shared `ErrorResponse` (`code`, `reason`, `message`, `details`, `request_id`, `fields[]`) and constructors (`not_found`, `forbidden`, `conflict(reason, message)`, `invalid(fields)`, `rate_limited(retry_after)`), mapped to the generated response types (openapi/crm/docs/09-api-conventions.md §6).
//!
//! Story: CRM-RT-01 (docs/crm/stories/).

use std::fmt;

/// One field-level validation failure inside a 422 `validation_failed`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

impl FieldError {
    #[must_use]
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}

/// Any failure a CRM service returns. `to_json` renders it as the shared `ErrorResponse` body that every CRM spec
/// declares; the controller adds the `X-Request-Id` header with the same `request_id`. Clients branch on `reason`,
/// never on `message`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiError {
    pub status: u16,
    pub reason: String,
    pub message: String,
    pub details: Option<String>,
    pub fields: Vec<FieldError>,
}

impl ApiError {
    /// A record the caller may see does not exist (404). A record in another workspace also answers this, never 403,
    /// so ids leak nothing (`09-api-conventions.md` §3).
    #[must_use]
    pub fn not_found() -> Self {
        Self {
            status: 404,
            reason: "not_found".to_owned(),
            message: "The record does not exist.".to_owned(),
            details: None,
            fields: Vec::new(),
        }
    }

    /// 403 with a stable reason (`workspace_forbidden`, `role_forbidden`, `owner_required`, ...).
    #[must_use]
    pub fn forbidden(reason: &str) -> Self {
        let message = match reason {
            "workspace_forbidden" => "The caller is not a member of this workspace.",
            "role_forbidden" => "The caller's role does not allow this operation.",
            _ => "The caller is not allowed to perform this operation.",
        };
        Self {
            status: 403,
            reason: reason.to_owned(),
            message: message.to_owned(),
            details: None,
            fields: Vec::new(),
        }
    }

    /// 401 `unauthenticated`: a request reached a handler without verified claims. The gateway rejects these before
    /// dispatch, so this is the fail-closed path (`RequestContext::from_request` answers it).
    #[must_use]
    pub fn unauthenticated() -> Self {
        Self {
            status: 401,
            reason: "unauthenticated".to_owned(),
            message: "Missing verified token claims.".to_owned(),
            details: None,
            fields: Vec::new(),
        }
    }

    /// 409 with the domain reason the contract names (`in_use`, `invalid_state`, `duplicate`, ...).
    #[must_use]
    pub fn conflict(reason: &str, message: &str) -> Self {
        Self {
            status: 409,
            reason: reason.to_owned(),
            message: message.to_owned(),
            details: None,
            fields: Vec::new(),
        }
    }

    /// 422 `validation_failed` with one entry per rejected field.
    #[must_use]
    pub fn invalid(fields: Vec<FieldError>) -> Self {
        Self {
            status: 422,
            reason: "validation_failed".to_owned(),
            message: "Some fields failed validation.".to_owned(),
            details: None,
            fields,
        }
    }

    /// 429 `rate_limited`. The seconds are kept in `details` (the only free-text slot `ErrorResponse` has);
    /// `retry_after_secs` reads them back for the `Retry-After` header.
    #[must_use]
    pub fn rate_limited(retry_after_secs: u32) -> Self {
        Self {
            status: 429,
            reason: "rate_limited".to_owned(),
            message: "Too many requests; retry after the indicated delay.".to_owned(),
            details: Some(retry_after_secs.to_string()),
            fields: Vec::new(),
        }
    }

    /// The `Retry-After` value for a 429, or `None` when it was not set by `rate_limited`.
    #[must_use]
    pub fn retry_after_secs(&self) -> Option<u32> {
        if self.reason == "rate_limited" {
            self.details.as_deref().and_then(|raw| raw.parse().ok())
        } else {
            None
        }
    }

    /// The `ErrorResponse` body. Optional keys appear only when set: `details`, `request_id`, and a non-empty `fields`.
    #[must_use]
    pub fn to_json(&self, request_id: Option<&str>) -> serde_json::Value {
        let mut body = serde_json::Map::new();
        body.insert("code".to_owned(), serde_json::json!(i64::from(self.status)));
        body.insert(
            "reason".to_owned(),
            serde_json::Value::String(self.reason.clone()),
        );
        body.insert(
            "message".to_owned(),
            serde_json::Value::String(self.message.clone()),
        );
        if let Some(details) = &self.details {
            body.insert(
                "details".to_owned(),
                serde_json::Value::String(details.clone()),
            );
        }
        if let Some(request_id) = request_id {
            body.insert(
                "request_id".to_owned(),
                serde_json::Value::String(request_id.to_owned()),
            );
        }
        if !self.fields.is_empty() {
            let fields = self
                .fields
                .iter()
                .map(|field| {
                    serde_json::json!({
                        "field": field.field,
                        "message": field.message,
                    })
                })
                .collect();
            body.insert("fields".to_owned(), serde_json::Value::Array(fields));
        }
        serde_json::Value::Object(body)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}: {}", self.status, self.reason, self.message)
    }
}

impl std::error::Error for ApiError {}

#[cfg(test)]
mod tests {
    use super::*;

    /// The contract pattern `^[a-z][a-z0-9_.]*$` on `ErrorResponse.reason`.
    fn is_contract_reason(reason: &str) -> bool {
        let mut chars = reason.chars();
        let Some(first) = chars.next() else {
            return false;
        };
        first.is_ascii_lowercase()
            && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '.')
    }

    /// Validate a rendered body against the `ErrorResponse` constraints of the CRM specs: required `code` (integer),
    /// `reason` (contract pattern) and `message` (string); optional `details` and `request_id` (strings) and
    /// `fields` (objects carrying exactly `field` and `message`, both required strings).
    fn assert_valid_error_response(body: &serde_json::Value) {
        let object = body.as_object().expect("body is an object");
        assert!(
            object
                .get("code")
                .and_then(serde_json::Value::as_i64)
                .is_some(),
            "code must be an integer: {body}"
        );
        let reason = object
            .get("reason")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_else(|| panic!("reason must be a present string: {body}"));
        assert!(
            is_contract_reason(reason),
            "reason `{reason}` breaks ^[a-z][a-z0-9_.]*$: {body}"
        );
        assert!(
            object
                .get("message")
                .and_then(serde_json::Value::as_str)
                .is_some(),
            "message must be a string: {body}"
        );
        for key in ["details", "request_id"] {
            if let Some(value) = object.get(key) {
                assert!(
                    value.is_string(),
                    "{key} must be a string when present: {body}"
                );
            }
        }
        if let Some(fields) = object.get("fields") {
            let items = fields.as_array().expect("fields must be an array");
            assert!(
                !items.is_empty(),
                "fields is omitted when empty, never sent empty"
            );
            for item in items {
                let item = item.as_object().expect("a field entry is an object");
                assert!(
                    item.get("field")
                        .and_then(serde_json::Value::as_str)
                        .is_some(),
                    "fields[].field is required: {item:?}"
                );
                assert!(
                    item.get("message")
                        .and_then(serde_json::Value::as_str)
                        .is_some(),
                    "fields[].message is required: {item:?}"
                );
                assert_eq!(
                    item.len(),
                    2,
                    "fields[] carries only field and message: {item:?}"
                );
            }
        }
        let known = [
            "code",
            "reason",
            "message",
            "details",
            "request_id",
            "fields",
        ];
        for key in object.keys() {
            assert!(
                known.contains(&key.as_str()),
                "unknown ErrorResponse key {key}"
            );
        }
    }

    fn all_constructors() -> Vec<(&'static str, ApiError)> {
        vec![
            ("not_found", ApiError::not_found()),
            (
                "workspace_forbidden",
                ApiError::forbidden("workspace_forbidden"),
            ),
            ("role_forbidden", ApiError::forbidden("role_forbidden")),
            ("unauthenticated", ApiError::unauthenticated()),
            (
                "conflict",
                ApiError::conflict("in_use", "This snippet is used by 3 templates."),
            ),
            (
                "invalid",
                ApiError::invalid(vec![FieldError::new("key", "already exists")]),
            ),
            ("rate_limited", ApiError::rate_limited(30)),
        ]
    }

    /// The verbatim `    ErrorResponse:` schema block of a spec, up to the next key at 2- or 4-space indent
    /// (a sibling schema, or the section after `schemas:`).
    fn error_response_schema(openapi_yaml: &str) -> Option<String> {
        let marker = "\n    ErrorResponse:\n";
        let start = openapi_yaml.find(marker)?;
        let rest = &openapi_yaml[start + marker.len()..];
        let mut block = String::from("    ErrorResponse:\n");
        for line in rest.lines() {
            let indent = line.len() - line.trim_start().len();
            let trimmed = line.trim();
            let is_boundary = matches!(indent, 2 | 4)
                && trimmed.ends_with(':')
                && !trimmed.contains(' ')
                && trimmed
                    .trim_end_matches(':')
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_')
                && !trimmed.is_empty();
            if is_boundary {
                break;
            }
            block.push_str(line);
            block.push('\n');
        }
        Some(block)
    }

    /// The nine CRM services this story set builds (docs/crm/README.md). The legacy `accounts`/`pipeline`/
    /// `conversion` and monolith-era specs under `openapi/crm/` keep the older error shape and are out of scope.
    const SERVICES: [&str; 9] = [
        "audience",
        "messaging",
        "automation",
        "sales",
        "acquisition",
        "insights",
        "workspace",
        "edge",
        "intake",
    ];

    fn crm_spec_paths() -> Vec<std::path::PathBuf> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("openapi/crm");
        SERVICES
            .iter()
            .map(|service| {
                let path = root.join(service).join("openapi.yaml");
                assert!(
                    path.is_file(),
                    "the CRM service {} has no openapi.yaml at {}",
                    service,
                    path.display()
                );
                path
            })
            .collect()
    }

    #[test]
    fn constructors_use_stable_status_and_reason() {
        let expected = [
            ("not_found", 404_u16, "not_found"),
            ("workspace_forbidden", 403, "workspace_forbidden"),
            ("role_forbidden", 403, "role_forbidden"),
            ("unauthenticated", 401, "unauthenticated"),
            ("conflict", 409, "in_use"),
            ("invalid", 422, "validation_failed"),
            ("rate_limited", 429, "rate_limited"),
        ];
        for ((name, error), (want_name, status, reason)) in
            all_constructors().into_iter().zip(expected)
        {
            assert_eq!(name, want_name);
            assert_eq!(error.status, status, "{name} status");
            assert_eq!(error.reason, reason, "{name} reason");
            assert!(!error.message.is_empty(), "{name} carries a message");
        }
    }

    #[test]
    fn to_json_omits_unset_optionals() {
        let body = ApiError::not_found().to_json(None);
        let object = body.as_object().expect("object");
        assert_eq!(object["code"], 404);
        assert_eq!(object["reason"], "not_found");
        assert!(object.get("request_id").is_none());
        assert!(object.get("details").is_none());
        assert!(object.get("fields").is_none());
    }

    #[test]
    fn to_json_carries_request_id_details_and_fields_when_set() {
        let mut error = ApiError::invalid(vec![FieldError::new("key", "already exists")]);
        error.details = Some("the key must be unique in the workspace".to_owned());
        let body = error.to_json(Some("01JC9V4M2Q8T7R6P5N4K3J2H1G"));
        let object = body.as_object().expect("object");
        assert_eq!(object["request_id"], "01JC9V4M2Q8T7R6P5N4K3J2H1G");
        assert_eq!(object["details"], "the key must be unique in the workspace");
        assert_eq!(object["fields"][0]["field"], "key");
        assert_eq!(object["fields"][0]["message"], "already exists");
    }

    #[test]
    fn rate_limited_carries_retry_after() {
        let error = ApiError::rate_limited(30);
        assert_eq!(error.retry_after_secs(), Some(30));
        assert_eq!(ApiError::not_found().retry_after_secs(), None);
        assert_eq!(error.to_json(None)["details"], "30");
    }

    #[test]
    fn api_error_is_a_std_error() {
        let error = ApiError::conflict("in_use", "used");
        let as_error: &dyn std::error::Error = &error;
        assert!(as_error.to_string().starts_with("409 in_use"));
    }

    #[test]
    fn every_service_declares_the_identical_error_response_schema() {
        let paths = crm_spec_paths();
        assert!(
            paths.len() >= 9,
            "expected every CRM service spec, found {}",
            paths.len()
        );
        let mut first: Option<(String, String)> = None;
        for path in &paths {
            let text = std::fs::read_to_string(path).expect("spec is readable");
            let schema = error_response_schema(&text)
                .unwrap_or_else(|| panic!("{} declares no ErrorResponse schema", path.display()));
            assert!(
                schema.contains("pattern: ^[a-z][a-z0-9_.]*$"),
                "{} ErrorResponse.reason must carry the contract pattern",
                path.display()
            );
            match &first {
                Some((first_path, first_schema)) => assert_eq!(
                    schema,
                    *first_schema,
                    "ErrorResponse differs between {} and {}",
                    first_path,
                    path.display()
                ),
                None => first = Some((path.display().to_string(), schema)),
            }
        }
    }

    #[test]
    fn every_constructor_json_validates_against_the_error_response_schema() {
        for (_name, error) in all_constructors() {
            assert_valid_error_response(&error.to_json(None));
            assert_valid_error_response(&error.to_json(Some("req-1")));
        }
    }

    #[test]
    fn display_shows_status_reason_and_message() {
        assert_eq!(
            ApiError::not_found().to_string(),
            "404 not_found: The record does not exist."
        );
    }
}
