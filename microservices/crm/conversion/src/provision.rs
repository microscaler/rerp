//! One-off: make the pipeline's Ed25519 signing key and store it in Secret Manager (ADR 003).
//!
//! Run by a Kubernetes Job with the RERP image itself (`CRM_PROVISION_CONVERSION_KEY=<project>/<secret>`), as an
//! identity that may create that secret and add versions but never read one. Stores one JSON value:
//! `{RERP_CRM_CONVERSION_KEY_ID: "k1", RERP_CRM_CONVERSION_SIGNING_KEY: <base64 seed>,
//!   RERP_CRM_CONVERSION_VERIFY_KEYS: "k1:<base64 public key>"}`. Idempotent: a secret that already holds an enabled
//! version is left alone. Waits out 403 (grants still propagating), 429 and 5xx for up to `WAIT_SECS` (default 3000).
//! Nothing secret is printed.

use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use ed25519_dalek::SigningKey;
use http_legacy::Method;
use may_minihttp::client::{Client, RedirectPolicy};
use serde_json::{json, Value};
use std::io::Read;
use std::time::{Duration, Instant};

const METADATA_TOKEN: &str =
    "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";

struct Sm {
    client: Client,
    base: String,
}

impl Sm {
    fn token(&self) -> Result<String, String> {
        let rsp = self
            .client
            .request(Method::GET, METADATA_TOKEN)
            .map_err(|e| e.to_string())?
            .header_str("metadata-flavor", "Google")
            .map_err(|e| e.to_string())?
            .send()
            .map_err(|e| format!("metadata: {e}"))?;
        let v: Value = serde_json::from_slice(rsp.body()).map_err(|e| format!("metadata: {e}"))?;
        v.get("access_token")
            .and_then(Value::as_str)
            .map(str::to_string)
            .ok_or_else(|| "metadata: no token".into())
    }

    /// (status, body)
    fn call(
        &self,
        method: Method,
        url: &str,
        body: Option<&Value>,
    ) -> Result<(u16, Value), String> {
        let mut req = self
            .client
            .request(method, url)
            .map_err(|e| e.to_string())?
            .header_str("authorization", &format!("Bearer {}", self.token()?))
            .map_err(|e| e.to_string())?
            .header_str("content-type", "application/json")
            .map_err(|e| e.to_string())?;
        if let Some(b) = body {
            req = req.body(b.to_string().into_bytes());
        }
        let rsp = req.send().map_err(|e| e.to_string())?;
        let status = rsp.status().as_u16();
        let v = serde_json::from_slice(rsp.body()).unwrap_or(Value::Null);
        Ok((status, v))
    }
}

fn transient(status: u16) -> bool {
    status == 403 || status == 429 || status >= 500
}

/// The JSON value stored for a seed (also used by tests).
pub fn secret_value(seed: &[u8; 32]) -> Value {
    let key = SigningKey::from_bytes(seed);
    json!({
        "RERP_CRM_CONVERSION_KEY_ID": "k1",
        "RERP_CRM_CONVERSION_SIGNING_KEY": B64.encode(seed),
        "RERP_CRM_CONVERSION_VERIFY_KEYS": format!("k1:{}", B64.encode(key.verifying_key().to_bytes())),
    })
}

fn seed() -> Result<[u8; 32], String> {
    let mut seed = [0u8; 32];
    std::fs::File::open("/dev/urandom")
        .and_then(|mut f| f.read_exact(&mut seed))
        .map_err(|e| format!("no randomness: {e}"))?;
    Ok(seed)
}

/// `spec` is `<project>/<secret>`. Returns what it did.
pub fn run(spec: &str) -> Result<String, String> {
    let (project, secret) = spec.split_once('/').ok_or("expected <project>/<secret>")?;
    let wait = std::env::var("WAIT_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000u64);
    let deadline = Instant::now() + Duration::from_secs(wait);
    let client = Client::builder()
        .redirect_policy(RedirectPolicy::None)
        .connect_timeout(Duration::from_secs(5))
        .request_timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| e.to_string())?;
    let sm = Sm {
        client,
        base: format!("https://secretmanager.googleapis.com/v1/projects/{project}/secrets"),
    };
    let retry = |what: &str| -> Result<(), String> {
        if Instant::now() >= deadline {
            return Err(format!("gave up: {what}"));
        }
        println!("waiting: {what}");
        std::thread::sleep(Duration::from_secs(30));
        Ok(())
    };

    // 1. Does the secret hold an enabled version?
    let exists = loop {
        let url = format!("{}/{secret}/versions?filter=state:ENABLED", sm.base);
        match sm.call(Method::GET, &url, None) {
            Ok((200, v)) => {
                if v.get("versions")
                    .and_then(Value::as_array)
                    .is_some_and(|a| !a.is_empty())
                {
                    return Ok(format!(
                        "secret {secret} already holds a key: nothing to do"
                    ));
                }
                break true;
            }
            Ok((404, _)) => break false,
            Ok((s, v)) if transient(s) => retry(&format!(
                "secret {secret} answered {s} {}",
                v["error"]["message"]
            ))?,
            Ok((s, v)) => {
                return Err(format!(
                    "secret {secret} answered {s} {}",
                    v["error"]["message"]
                ))
            }
            Err(e) => retry(&e)?,
        }
    };
    // 2. Create it.
    if !exists {
        let body = json!({"replication": {"automatic": {}}, "labels": {"product": "rerp", "owner": "crm-conversion"}});
        loop {
            match sm.call(
                Method::POST,
                &format!("{}?secretId={secret}", sm.base),
                Some(&body),
            ) {
                Ok((200, _)) | Ok((409, _)) => break,
                Ok((s, v)) if transient(s) => retry(&format!(
                    "creating {secret} answered {s} {}",
                    v["error"]["message"]
                ))?,
                Ok((s, v)) => {
                    return Err(format!(
                        "creating {secret} answered {s} {}",
                        v["error"]["message"]
                    ))
                }
                Err(e) => retry(&e)?,
            }
        }
        println!("secret {secret}: created");
    }
    // 3. The key, stored as the first version.
    let value = secret_value(&seed()?);
    let body = json!({"payload": {"data": B64.encode(value.to_string())}});
    loop {
        match sm.call(
            Method::POST,
            &format!("{}/{secret}:addVersion", sm.base),
            Some(&body),
        ) {
            Ok((200, _)) => break,
            Ok((s, v)) if transient(s) => retry(&format!(
                "storing the key answered {s} {}",
                v["error"]["message"]
            ))?,
            Ok((s, v)) => {
                return Err(format!(
                    "storing the key answered {s} {}",
                    v["error"]["message"]
                ))
            }
            Err(e) => retry(&e)?,
        }
    }
    Ok(format!("secret {secret}: key k1 stored"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Keys, Sealer};

    #[test]
    fn the_stored_value_is_a_working_key_pair() {
        let v = secret_value(&[3u8; 32]);
        let sealer = Sealer::from_seed_b64(
            v["RERP_CRM_CONVERSION_KEY_ID"].as_str().unwrap(),
            v["RERP_CRM_CONVERSION_SIGNING_KEY"].as_str().unwrap(),
        )
        .unwrap();
        let keys = Keys::parse(v["RERP_CRM_CONVERSION_VERIFY_KEYS"].as_str().unwrap()).unwrap();
        assert!(!keys.is_empty());
        assert_eq!(
            format!("k1:{}", sealer.public_b64()),
            v["RERP_CRM_CONVERSION_VERIFY_KEYS"]
        );
    }

    #[test]
    fn randomness_gives_distinct_seeds() {
        assert_ne!(seed().unwrap(), seed().unwrap());
    }
}
