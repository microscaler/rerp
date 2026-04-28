// User-owned controller for handler 'create_payment_application'.

// Native untyped proxy route matching JSF Compliance
use brrtrouter::dispatcher::{HandlerRequest, HandlerResponse};

pub fn handle(req: HandlerRequest) -> HandlerResponse {
    // Build path matching the spec
    let path_str = "/api/accounts-receivable/payments/applications";
    let mut resolved_path = path_str.to_string();
    for (k, v) in &req.path_params {
        let mut needle = String::new();
        needle.push('{');
        needle.push_str(k.as_ref());
        needle.push('}');
        resolved_path = resolved_path.replace(&needle, v.as_str());
    }

    if !req.query_params.is_empty() {
        let mut qs = String::new();
        for (i, (k, v)) in req.query_params.iter().enumerate() {
            if i > 0 {
                qs.push('&')
            } else {
                qs.push('?')
            }
            qs.push_str(k.as_ref());
            qs.push('=');
            qs.push_str(v.as_str());
        }
        resolved_path.push_str(&qs);
    }

    let service_name_upper = "accounts-receivable".to_uppercase();
    let port_env_key = format!("{}_PORT", service_name_upper);
    let port: u16 = std::env::var(&port_env_key)
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(80);

    // JSF Rule 208: Catch all protocol failures gracefully
    let domain_suffix =
        std::env::var("SERVICE_DOMAIN").unwrap_or_else(|_| "svc.cluster.local".to_string());
    let service_fqdn = format!("{}.{}", "accounts-receivable", domain_suffix);

    let target_ip = match std::net::ToSocketAddrs::to_socket_addrs(&(service_fqdn.as_str(), port)) {
        Ok(mut iter) => match iter.next() {
            Some(addr) => addr,
            None => return HandlerResponse::error(502, "DNS resolution empty"),
        },
        Err(e) => return HandlerResponse::error(502, &format!("DNS error: {}", e)),
    };

    let url = match format!("{}", resolved_path).parse::<http_legacy::Uri>() {
        Ok(uri) => uri,
        Err(e) => return HandlerResponse::error(500, &format!("Invalid Proxy URI: {}", e)),
    };

    let method = match http_legacy::Method::from_bytes(req.method.as_str().as_bytes()) {
        Ok(m) => m,
        Err(e) => return HandlerResponse::error(400, &format!("Invalid Method: {}", e)),
    };

    thread_local! {
        static CLIENT_CACHE: std::cell::RefCell<Option<may_http::client::HttpClient>> = std::cell::RefCell::new(None);
    }

    let proxy_result = CLIENT_CACHE.with(|cache| -> Result<HandlerResponse, String> {
        let mut client_opt = cache.borrow_mut();
        if client_opt.is_none() {
            match may_http::client::HttpClient::connect(target_ip) {
                Ok(client) => *client_opt = Some(client),
                Err(e) => return Err(format!("Failed to connect to proxy target: {}", e)),
            }
        }
        let client = client_opt
            .as_mut()
            .expect("HttpClient must be present after connect");

        fn skip_forward_request_header(name: &str) -> bool {
            name.eq_ignore_ascii_case("host")
                || name.eq_ignore_ascii_case("connection")
                || name.eq_ignore_ascii_case("content-length")
                || name.eq_ignore_ascii_case("transfer-encoding")
                || name.eq_ignore_ascii_case("upgrade")
                || name.eq_ignore_ascii_case("te")
                || name.eq_ignore_ascii_case("trailer")
                || name.eq_ignore_ascii_case("proxy-connection")
        }

        let mut proxy_req = client.new_request(method.clone(), url);
        for (hk, hv) in &req.headers {
            let name = hk.as_ref();
            if skip_forward_request_header(name) {
                continue;
            }
            match (
                http_legacy::header::HeaderName::from_bytes(name.as_bytes()),
                http_legacy::header::HeaderValue::from_str(hv.as_str()),
            ) {
                (Ok(hname), Ok(hval)) => {
                    proxy_req.headers_mut().insert(hname, hval);
                }
                _ => {}
            }
        }
        if proxy_req
            .headers()
            .get(http_legacy::header::ACCEPT)
            .is_none()
        {
            if let Ok(safe_accept) = http_legacy::header::HeaderValue::from_str("application/json")
            {
                proxy_req
                    .headers_mut()
                    .insert(http_legacy::header::ACCEPT, safe_accept);
            }
        }

        let mut buf = Vec::with_capacity(1024); // JSF-compliant: allocating maximum socket transit size avoids multi-page OS interrupts

        if let Some(body_json) = &req.body {
            let body_bytes = match serde_json::to_vec(body_json) {
                Ok(b) => b,
                Err(e) => return Err(format!("Failed to serialize body: {}", e)),
            };
            if let Err(e) = proxy_req.send(&body_bytes) {
                *client_opt = None;
                return Err(format!("Failed to send proxy body: {}", e));
            }
        }
        let mut rsp = match client.send_request(proxy_req) {
            Ok(r) => r,
            Err(e) => {
                *client_opt = None;
                return Err(format!("Failed to send proxy request: {}", e));
            }
        };

        match std::io::Read::read_to_end(&mut rsp, &mut buf) {
            Ok(_) => {}
            Err(e) => return Err(format!("Failed to read proxy response: {}", e)),
        }

        // Deserialize response text into serde JSON
        let body_json: serde_json::Value = match serde_json::from_slice(&buf) {
            Ok(v) => v,
            Err(_) => {
                let text = String::from_utf8_lossy(&buf).into_owned();
                serde_json::json!({ "text_response": text })
            }
        };

        let status = rsp.status().as_u16();
        fn skip_forward_response_header(name: &str) -> bool {
            name.eq_ignore_ascii_case("connection")
                || name.eq_ignore_ascii_case("transfer-encoding")
                || name.eq_ignore_ascii_case("keep-alive")
                || name.eq_ignore_ascii_case("upgrade")
                || name.eq_ignore_ascii_case("trailer")
                || name.eq_ignore_ascii_case("proxy-authenticate")
        }
        let mut out_headers = brrtrouter::dispatcher::HeaderVec::new();
        for (name, value) in rsp.headers().iter() {
            if skip_forward_response_header(name.as_str()) {
                continue;
            }
            if let Ok(s) = value.to_str() {
                out_headers.push((std::sync::Arc::from(name.as_str()), s.to_string()));
            }
        }
        if !out_headers
            .iter()
            .any(|(k, _)| k.eq_ignore_ascii_case("content-type"))
        {
            out_headers.push((
                std::sync::Arc::from("content-type"),
                "application/json".to_string(),
            ));
        }
        Ok(HandlerResponse::new(status, out_headers, body_json))
    });

    match proxy_result {
        Ok(res) => res,
        Err(err_msg) => HandlerResponse::error(502, &err_msg),
    }
}
