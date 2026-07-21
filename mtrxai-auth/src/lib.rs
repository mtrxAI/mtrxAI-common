//! Constant-time string comparison and bearer-token helpers.

/// Constant-time equality for UTF-8 strings (length mismatch returns false early).
pub fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.bytes()
        .zip(b.bytes())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y))
        == 0
}

/// Strip optional `Bearer ` / `bearer ` prefix and trim.
pub fn strip_bearer(header_value: &str) -> &str {
    header_value
        .strip_prefix("Bearer ")
        .or_else(|| header_value.strip_prefix("bearer "))
        .unwrap_or(header_value)
        .trim()
}

/// Verify a Bearer Authorization header against an expected token.
pub fn verify_bearer_token(header_value: Option<&str>, expected: &str) -> bool {
    let Some(raw) = header_value else {
        return false;
    };
    constant_time_eq(strip_bearer(raw), expected)
}

/// Verify a raw header value (e.g. `x-admin-key`) against an expected secret.
pub fn verify_header_token(provided: Option<&str>, expected: &str) -> bool {
    let Some(raw) = provided else {
        return false;
    };
    constant_time_eq(raw.trim(), expected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bearer_ok() {
        assert!(verify_bearer_token(Some("Bearer secret"), "secret"));
        assert!(!verify_bearer_token(Some("Bearer other"), "secret"));
        assert!(!verify_bearer_token(None, "secret"));
    }
}
