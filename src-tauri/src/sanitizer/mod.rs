use lazy_static::lazy_static;
use regex::{Captures, Regex};
use serde_json::Value;

/// Helper: returns true when `ip` is a valid IPv4 string and is *public* (not localhost or private ranges).
fn is_public_ipv4(ip: &str) -> bool {
    // Parse octets
    let octets: Vec<&str> = ip.split('.').collect();
    if octets.len() != 4 {
        return false;
    }
    let mut nums = [0u8; 4];
    for (i, s) in octets.iter().enumerate() {
        // reject empty or non-numeric
        if s.is_empty() {
            return false;
        }
        // parse as u8 (reject >255)
        match s.parse::<u8>() {
            Ok(n) => nums[i] = n,
            Err(_) => return false,
        }
    }

    // 127.* => loopback
    if nums[0] == 127 {
        return false;
    }
    // 10.* => private
    if nums[0] == 10 {
        return false;
    }
    // 192.168.* => private
    if nums[0] == 192 && nums[1] == 168 {
        return false;
    }
    // 172.16.0.0 - 172.31.255.255 => private
    if nums[0] == 172 && (16..=31).contains(&nums[1]) {
        return false;
    }
    // Link-local 169.254.*
    if nums[0] == 169 && nums[1] == 254 {
        return false;
    }
    // Carrier-grade NAT 100.64.0.0/10
    if nums[0] == 100 && (64..=127).contains(&nums[1]) {
        return false;
    }
    // If none of the above, treat as public
    true
}

lazy_static! {
    // API Keys, tokens, secrets (case-insensitive)
    static ref API_KEY_PATTERN: Regex = Regex::new(
        r#"(?i)(api[_-]?key|token|secret|password|bearer|authorization)\s*[:=]\s*['"]?([A-Za-z0-9_\-./+]{16,})['"]?"#
    ).unwrap();

    // Email addresses
    static ref EMAIL_PATTERN: Regex = Regex::new(
        r#"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b"#
    ).unwrap();

    // IPv4 addresses (match any IPv4-like sequence; filter private/local addresses in code)
    static ref IPV4_PATTERN: Regex = Regex::new(
        r#"\b(?:[0-9]{1,3}\.){3}[0-9]{1,3}\b"#
    ).unwrap();

    // Private keys — allow dot to match newlines with (?s) and use non-greedy match
    static ref PRIVATE_KEY_PATTERN: Regex = Regex::new(
        r"(?s)-----BEGIN [A-Z ]+PRIVATE KEY-----.+?-----END [A-Z ]+PRIVATE KEY-----"
    ).unwrap();

    // Credit card numbers (simple pattern)
    static ref CC_PATTERN: Regex = Regex::new(
        r#"\b\d{4}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}\b"#
    ).unwrap();
}

pub fn sanitize_text(text: &str) -> String {
    let mut sanitized = text.to_string();

    // Replace API keys keeping the captured key name ($1)
    sanitized = API_KEY_PATTERN
        .replace_all(&sanitized, "$1: [REDACTED_KEY]")
        .to_string();

    // Emails
    sanitized = EMAIL_PATTERN
        .replace_all(&sanitized, "[EMAIL_REDACTED]")
        .to_string();

    // IPv4s: use closure to check whether the match is public (only redact public IPv4s)
    sanitized = IPV4_PATTERN
        .replace_all(&sanitized, |caps: &Captures| {
            let ip = caps.get(0).map(|m| m.as_str()).unwrap_or("");
            if is_public_ipv4(ip) {
                "[IP_REDACTED]".to_string()
            } else {
                // return original (keep localhost/private addresses untouched)
                ip.to_string()
            }
        })
        .to_string();

    // Private keys
    sanitized = PRIVATE_KEY_PATTERN
        .replace_all(&sanitized, "[PRIVATE_KEY_REDACTED]")
        .to_string();

    // Credit cards
    sanitized = CC_PATTERN
        .replace_all(&sanitized, "[CC_REDACTED]")
        .to_string();

    sanitized
}

pub fn sanitize_json(value: &Value) -> Value {
    match value {
        Value::String(s) => Value::String(sanitize_text(s)),
        Value::Array(arr) => Value::Array(
            arr.iter().map(|v| sanitize_json(v)).collect::<Vec<Value>>()
        ),
        Value::Object(obj) => Value::Object(
            obj.iter()
                .map(|(k, v)| (k.clone(), sanitize_json(v)))
                .collect::<serde_json::Map<String, Value>>()
        ),
        other => other.clone(),
    }
}
