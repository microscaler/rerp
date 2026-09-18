//! Narrow S3-compatible object operations over the coroutine-native may client.
//!
//! This intentionally implements only immutable PUT, verification HEAD and
//! presigned GET. It is not an AWS SDK or general object-storage abstraction.

use chrono::{DateTime, Duration, Utc};
use hmac::{Hmac, Mac};
use http_legacy::Method;
use may_minihttp::client::{Client, RedirectPolicy};
use sha2::{Digest, Sha256};
use std::sync::OnceLock;
use std::time::Duration as StdDuration;
use url::Url;

type HmacSha256 = Hmac<Sha256>;
const EMPTY_SHA256: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

pub struct ObjectStore {
    client: Client,
    endpoint: String,
    public_endpoint: String,
    region: String,
    bucket: String,
    access_key: String,
    secret_key: String,
    presign_seconds: i64,
}

#[derive(Clone, Debug)]
pub struct PresignedObject {
    pub url: String,
    pub expires_at: DateTime<Utc>,
}

static STORE: OnceLock<Result<ObjectStore, String>> = OnceLock::new();

pub fn store() -> Result<&'static ObjectStore, String> {
    STORE
        .get_or_init(ObjectStore::from_env)
        .as_ref()
        .map_err(Clone::clone)
}

impl ObjectStore {
    fn from_env() -> Result<Self, String> {
        let in_kubernetes = std::env::var_os("KUBERNETES_SERVICE_HOST").is_some();
        let endpoint = std::env::var("RERP_OBJECT_STORE_ENDPOINT").unwrap_or_else(|_| {
            if in_kubernetes {
                "http://minio.data.svc.cluster.local:9000".to_string()
            } else {
                "http://127.0.0.1:9000".to_string()
            }
        });
        let public_endpoint =
            std::env::var("RERP_OBJECT_STORE_PUBLIC_ENDPOINT").unwrap_or_else(|_| endpoint.clone());
        validate_endpoint(&endpoint)?;
        validate_endpoint(&public_endpoint)?;
        let access_key = required_env("RERP_OBJECT_STORE_ACCESS_KEY")?;
        let secret_key = required_env("RERP_OBJECT_STORE_SECRET_KEY")?;
        let bucket = std::env::var("RERP_OBJECT_STORE_BUCKET")
            .unwrap_or_else(|_| "rerp-accounting-documents".to_string());
        validate_bucket(&bucket)?;
        let presign_seconds = std::env::var("RERP_OBJECT_STORE_PRESIGN_SECONDS")
            .ok()
            .and_then(|value| value.parse::<i64>().ok())
            .filter(|value| (60..=900).contains(value))
            .unwrap_or(300);
        let client = Client::builder()
            .redirect_policy(RedirectPolicy::None)
            .connect_timeout(StdDuration::from_secs(3))
            .request_timeout(StdDuration::from_secs(10))
            .build()
            .map_err(|error| format!("object client configuration: {error}"))?;
        Ok(Self {
            client,
            endpoint,
            public_endpoint,
            region: std::env::var("RERP_OBJECT_STORE_REGION")
                .unwrap_or_else(|_| "us-east-1".to_string()),
            bucket,
            access_key,
            secret_key,
            presign_seconds,
        })
    }

    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    pub fn put_immutable(&self, key: &str, bytes: &[u8], checksum: &str) -> Result<(), String> {
        validate_key(key)?;
        let url = self.object_url(&self.endpoint, key)?;
        let now = Utc::now();
        let payload_hash = sha256_hex(bytes);
        if payload_hash != checksum {
            return Err("object checksum does not match payload".to_string());
        }
        let headers = vec![
            ("content-type", "application/pdf".to_string()),
            ("if-none-match", "*".to_string()),
            ("x-amz-meta-sha256", checksum.to_string()),
        ];
        let response =
            self.signed_request(Method::PUT, &url, bytes, &headers, &payload_hash, now)?;
        if response.status().is_success() {
            return Ok(());
        }
        if response.status().as_u16() == 412 {
            return self.verify_existing(key, checksum, bytes.len());
        }
        Err(format!(
            "immutable object PUT returned HTTP {}",
            response.status().as_u16()
        ))
    }

    fn verify_existing(&self, key: &str, checksum: &str, size: usize) -> Result<(), String> {
        let url = self.object_url(&self.endpoint, key)?;
        let response =
            self.signed_request(Method::HEAD, &url, &[], &[], EMPTY_SHA256, Utc::now())?;
        if !response.status().is_success() {
            return Err(format!(
                "existing object verification returned HTTP {}",
                response.status().as_u16()
            ));
        }
        let stored_checksum = response
            .headers()
            .get("x-amz-meta-sha256")
            .and_then(|value| value.to_str().ok());
        let stored_size = response
            .headers()
            .get("content-length")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<usize>().ok());
        if stored_checksum == Some(checksum) && stored_size == Some(size) {
            Ok(())
        } else {
            Err("content-addressed object exists with unexpected metadata".to_string())
        }
    }

    pub fn presigned_get(&self, key: &str) -> Result<PresignedObject, String> {
        validate_key(key)?;
        let now = Utc::now();
        self.presigned_get_at(key, now)
    }

    fn presigned_get_at(&self, key: &str, now: DateTime<Utc>) -> Result<PresignedObject, String> {
        let url = self.object_url(&self.public_endpoint, key)?;
        let parsed = Url::parse(&url).map_err(|error| format!("invalid object URL: {error}"))?;
        let host = canonical_host(&parsed)?;
        let date = now.format("%Y%m%d").to_string();
        let timestamp = now.format("%Y%m%dT%H%M%SZ").to_string();
        let scope = format!("{date}/{}/s3/aws4_request", self.region);
        let mut query = [
            ("X-Amz-Algorithm", "AWS4-HMAC-SHA256".to_string()),
            ("X-Amz-Credential", format!("{}/{}", self.access_key, scope)),
            ("X-Amz-Date", timestamp.clone()),
            ("X-Amz-Expires", self.presign_seconds.to_string()),
            ("X-Amz-SignedHeaders", "host".to_string()),
        ];
        query.sort_by(|left, right| left.0.cmp(right.0));
        let canonical_query = query
            .iter()
            .map(|(name, value)| format!("{}={}", percent_encode(name), percent_encode(value)))
            .collect::<Vec<_>>()
            .join("&");
        let canonical_request = format!(
            "GET\n{}\n{}\nhost:{}\n\nhost\nUNSIGNED-PAYLOAD",
            parsed.path(),
            canonical_query,
            host
        );
        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            timestamp,
            scope,
            sha256_hex(canonical_request.as_bytes())
        );
        let signature = hex(
            &signing_key(&self.secret_key, &date, &self.region),
            string_to_sign.as_bytes(),
        );
        Ok(PresignedObject {
            url: format!("{url}?{canonical_query}&X-Amz-Signature={signature}"),
            expires_at: now + Duration::seconds(self.presign_seconds),
        })
    }

    fn object_url(&self, endpoint: &str, key: &str) -> Result<String, String> {
        validate_key(key)?;
        let encoded_key = key
            .split('/')
            .map(percent_encode)
            .collect::<Vec<_>>()
            .join("/");
        Ok(format!(
            "{}/{}/{}",
            endpoint.trim_end_matches('/'),
            percent_encode(&self.bucket),
            encoded_key
        ))
    }

    fn signed_request(
        &self,
        method: Method,
        url: &str,
        body: &[u8],
        additional_headers: &[(&str, String)],
        payload_hash: &str,
        now: DateTime<Utc>,
    ) -> Result<may_minihttp::client::BufferedResponse, String> {
        let parsed = Url::parse(url).map_err(|error| format!("invalid object URL: {error}"))?;
        let host = canonical_host(&parsed)?;
        let date = now.format("%Y%m%d").to_string();
        let timestamp = now.format("%Y%m%dT%H%M%SZ").to_string();
        let mut headers = additional_headers.to_vec();
        headers.push(("host", host.clone()));
        headers.push(("x-amz-content-sha256", payload_hash.to_string()));
        headers.push(("x-amz-date", timestamp.clone()));
        headers.sort_by(|left, right| left.0.cmp(right.0));
        let canonical_headers = headers
            .iter()
            .map(|(name, value)| format!("{}:{}\n", name, value.trim()))
            .collect::<String>();
        let signed_headers = headers
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>()
            .join(";");
        let canonical_request = format!(
            "{}\n{}\n\n{}\n{}\n{}",
            method.as_str(),
            parsed.path(),
            canonical_headers,
            signed_headers,
            payload_hash
        );
        let scope = format!("{date}/{}/s3/aws4_request", self.region);
        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            timestamp,
            scope,
            sha256_hex(canonical_request.as_bytes())
        );
        let signature = hex(
            &signing_key(&self.secret_key, &date, &self.region),
            string_to_sign.as_bytes(),
        );
        let authorization = format!(
            "AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
            self.access_key, scope, signed_headers, signature
        );
        let mut request = self
            .client
            .request(method, url)
            .map_err(|error| format!("object request: {error}"))?;
        for (name, value) in headers {
            if name != "host" {
                request = request
                    .header_str(name, &value)
                    .map_err(|error| format!("object request header: {error}"))?;
            }
        }
        request
            .header_str("authorization", &authorization)
            .map_err(|error| format!("object authorization header: {error}"))?
            .body(body.to_vec())
            .send()
            .map_err(|error| format!("object request failed: {error}"))
    }
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn signing_key(secret: &str, date: &str, region: &str) -> Vec<u8> {
    let date_key = hmac(format!("AWS4{secret}").as_bytes(), date.as_bytes());
    let region_key = hmac(&date_key, region.as_bytes());
    let service_key = hmac(&region_key, b"s3");
    hmac(&service_key, b"aws4_request")
}

fn hmac(key: &[u8], value: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC accepts keys of any length");
    mac.update(value);
    mac.finalize().into_bytes().to_vec()
}

fn hex(key: &[u8], value: &[u8]) -> String {
    hmac(key, value)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn percent_encode(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(char::from(byte));
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    encoded
}

fn canonical_host(url: &Url) -> Result<String, String> {
    let host = url
        .host_str()
        .ok_or_else(|| "object URL has no host".to_string())?;
    Ok(match url.port() {
        Some(port) => format!("{host}:{port}"),
        None => host.to_string(),
    })
}

fn required_env(key: &str) -> Result<String, String> {
    std::env::var(key)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| format!("{key} is required"))
}

fn validate_endpoint(value: &str) -> Result<(), String> {
    let parsed =
        Url::parse(value).map_err(|error| format!("invalid object-store endpoint: {error}"))?;
    if !matches!(parsed.scheme(), "http" | "https")
        || parsed.host_str().is_none()
        || parsed.query().is_some()
        || parsed.fragment().is_some()
        || parsed.path() != "/"
    {
        return Err("object-store endpoint must be an absolute HTTP(S) origin without path, query or fragment".to_string());
    }
    Ok(())
}

fn validate_bucket(value: &str) -> Result<(), String> {
    if (3..=63).contains(&value.len())
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'-' | b'.')
        })
        && !value.starts_with(['-', '.'])
        && !value.ends_with(['-', '.'])
    {
        Ok(())
    } else {
        Err("invalid object-store bucket name".to_string())
    }
}

fn validate_key(value: &str) -> Result<(), String> {
    if value.is_empty()
        || value.starts_with('/')
        || value
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
    {
        Err("invalid object key".to_string())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_matches_standard_vector() {
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn encoding_and_validation_are_path_safe() {
        assert_eq!(percent_encode("a b/c"), "a%20b%2Fc");
        assert!(validate_key("entity/document/hash.pdf").is_ok());
        assert!(validate_key("../secret").is_err());
        assert!(validate_endpoint("http://minio:9000").is_ok());
        assert!(validate_endpoint("http://minio:9000/path").is_err());
    }
}
