//! A small Pub/Sub REST client for may services: publish, pull, ack, nack.
//!
//! Credentials are the pod's Workload Identity, read from the GKE metadata server. With `PUBSUB_EMULATOR_HOST` set
//! (local development), it talks plain HTTP to the emulator without credentials.

use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use http_legacy::Method;
use may_minihttp::client::{Client, RedirectPolicy};
use serde_json::{json, Value};
use std::time::{Duration, Instant};

const METADATA_TOKEN: &str =
    "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";

/// One pulled message.
#[derive(Clone, Debug)]
pub struct Received {
    pub ack_id: String,
    pub data: Vec<u8>,
    pub delivery_attempt: u32,
}

pub struct PubSub {
    client: Client,
    base: String,
    emulator: bool,
    token: String,
    token_until: Option<Instant>,
}

impl PubSub {
    pub fn new() -> Result<Self, String> {
        let emulator = std::env::var("PUBSUB_EMULATOR_HOST")
            .ok()
            .filter(|h| !h.trim().is_empty());
        let client = Client::builder()
            .redirect_policy(RedirectPolicy::None)
            .connect_timeout(Duration::from_secs(5))
            // A pull long-polls for up to about a minute.
            .request_timeout(Duration::from_secs(90))
            .build()
            .map_err(|e| format!("pubsub client: {e}"))?;
        Ok(match emulator {
            Some(host) => Self {
                client,
                base: format!("http://{host}"),
                emulator: true,
                token: String::new(),
                token_until: None,
            },
            None => Self {
                client,
                base: "https://pubsub.googleapis.com".into(),
                emulator: false,
                token: String::new(),
                token_until: None,
            },
        })
    }

    fn bearer(&mut self) -> Result<Option<String>, String> {
        if self.emulator {
            return Ok(None);
        }
        if self.token_until.is_some_and(|u| Instant::now() < u) {
            return Ok(Some(self.token.clone()));
        }
        let rsp = self
            .client
            .request(Method::GET, METADATA_TOKEN)
            .map_err(|e| format!("metadata: {e}"))?
            .header_str("metadata-flavor", "Google")
            .map_err(|e| format!("metadata: {e}"))?
            .send()
            .map_err(|e| format!("metadata: {e}"))?;
        if rsp.status().as_u16() != 200 {
            return Err(format!("metadata: HTTP {}", rsp.status().as_u16()));
        }
        let v: Value = serde_json::from_slice(rsp.body()).map_err(|e| format!("metadata: {e}"))?;
        let token = v
            .get("access_token")
            .and_then(Value::as_str)
            .ok_or("metadata: no access_token")?;
        let ttl = v.get("expires_in").and_then(Value::as_u64).unwrap_or(300);
        self.token = token.to_string();
        self.token_until = Some(Instant::now() + Duration::from_secs(ttl.saturating_sub(60)));
        Ok(Some(self.token.clone()))
    }

    fn call(&mut self, resource: &str, verb: &str, body: Value) -> Result<Value, String> {
        let bearer = self.bearer()?;
        let url = format!("{}/v1/{resource}:{verb}", self.base);
        let mut req = self
            .client
            .request(Method::POST, &url)
            .map_err(|e| format!("pubsub {verb}: {e}"))?
            .header_str("content-type", "application/json")
            .map_err(|e| format!("pubsub {verb}: {e}"))?;
        if let Some(t) = bearer {
            req = req
                .header_str("authorization", &format!("Bearer {t}"))
                .map_err(|e| format!("pubsub {verb}: {e}"))?;
        }
        let rsp = req
            .body(body.to_string().into_bytes())
            .send()
            .map_err(|e| format!("pubsub {verb}: {e}"))?;
        let status = rsp.status().as_u16();
        if !(200..300).contains(&status) {
            let text = String::from_utf8_lossy(rsp.body())
                .chars()
                .take(300)
                .collect::<String>();
            return Err(format!("pubsub {verb}: HTTP {status}: {text}"));
        }
        if rsp.body().is_empty() {
            return Ok(json!({}));
        }
        serde_json::from_slice(rsp.body()).map_err(|e| format!("pubsub {verb}: {e}"))
    }

    /// Publish one message to `projects/<p>/topics/<t>`; returns the message id.
    pub fn publish(
        &mut self,
        topic: &str,
        data: &[u8],
        attributes: &[(&str, &str)],
    ) -> Result<String, String> {
        let attrs: serde_json::Map<String, Value> = attributes
            .iter()
            .map(|(k, v)| (k.to_string(), Value::String(v.to_string())))
            .collect();
        let v = self.call(
            topic,
            "publish",
            json!({ "messages": [{ "data": B64.encode(data), "attributes": attrs }] }),
        )?;
        v.get("messageIds")
            .and_then(|ids| ids.get(0))
            .and_then(Value::as_str)
            .map(str::to_string)
            .ok_or_else(|| "pubsub publish: no message id".to_string())
    }

    /// Pull up to `max` messages from `projects/<p>/subscriptions/<s>`.
    pub fn pull(&mut self, subscription: &str, max: u32) -> Result<Vec<Received>, String> {
        let v = self.call(subscription, "pull", json!({ "maxMessages": max }))?;
        let mut out = Vec::new();
        for m in v
            .get("receivedMessages")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
        {
            let ack_id = m
                .get("ackId")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
            let raw = m
                .pointer("/message/data")
                .and_then(Value::as_str)
                .unwrap_or_default();
            // Undecodable data is still handed over (empty) so the caller can drop and ack it.
            let data = B64.decode(raw).unwrap_or_default();
            let delivery_attempt = m
                .get("deliveryAttempt")
                .and_then(Value::as_u64)
                .unwrap_or(1) as u32;
            if !ack_id.is_empty() {
                out.push(Received {
                    ack_id,
                    data,
                    delivery_attempt,
                });
            }
        }
        Ok(out)
    }

    pub fn ack(&mut self, subscription: &str, ack_ids: &[String]) -> Result<(), String> {
        if ack_ids.is_empty() {
            return Ok(());
        }
        self.call(subscription, "acknowledge", json!({ "ackIds": ack_ids }))
            .map(|_| ())
    }

    /// Hand the messages back for redelivery now (with the subscription's back-off and dead-lettering).
    pub fn nack(&mut self, subscription: &str, ack_ids: &[String]) -> Result<(), String> {
        if ack_ids.is_empty() {
            return Ok(());
        }
        self.call(
            subscription,
            "modifyAckDeadline",
            json!({ "ackIds": ack_ids, "ackDeadlineSeconds": 0 }),
        )
        .map(|_| ())
    }
}
