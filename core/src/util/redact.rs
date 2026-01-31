//! Utilities for redacting sensitive information from logs and output

use std::borrow::Cow;

/// Patterns that should be redacted in output
const SENSITIVE_PATTERNS: &[&str] = &[
    "password",
    "passwd",
    "secret",
    "token",
    "key",
    "credential",
    "auth",
];

/// Redact sensitive information from a string
/// 
/// This replaces values that look like they contain sensitive data
/// with "[REDACTED]"
pub fn redact_sensitive(input: &str) -> Cow<'_, str> {
    // Check if any sensitive pattern exists (case-insensitive)
    let lower = input.to_lowercase();
    let has_sensitive = SENSITIVE_PATTERNS.iter().any(|p| lower.contains(p));
    
    if !has_sensitive {
        return Cow::Borrowed(input);
    }

    // Redact key=value patterns
    let mut result = input.to_string();
    
    for pattern in SENSITIVE_PATTERNS {
        // Match patterns like "password=xxx" or "password: xxx"
        let patterns_to_check = [
            format!("{}=", pattern),
            format!("{} =", pattern),
            format!("{}:", pattern),
            format!("{} :", pattern),
        ];
        
        for prefix in patterns_to_check {
            if let Some(start) = result.to_lowercase().find(&prefix) {
                let value_start = start + prefix.len();
                // Find the end of the value (space, newline, or end of string)
                let value_end = result[value_start..]
                    .find(|c: char| c.is_whitespace() || c == ',' || c == ';' || c == '"' || c == '\'')
                    .map(|i| value_start + i)
                    .unwrap_or(result.len());
                
                if value_end > value_start {
                    result.replace_range(value_start..value_end, "[REDACTED]");
                }
            }
        }
    }

    Cow::Owned(result)
}

/// Redact a path that might contain sensitive information
pub fn redact_path(path: &str) -> String {
    // Redact home directory
    if let Ok(home) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
        if path.starts_with(&home) {
            return path.replacen(&home, "~", 1);
        }
    }
    path.to_string()
}

/// Redact IP addresses (replace last octet with xxx)
pub fn redact_ip(ip: &str) -> String {
    if let Some(last_dot) = ip.rfind('.') {
        let prefix = &ip[..last_dot];
        // Check if it looks like an IP
        if prefix.chars().all(|c| c.is_ascii_digit() || c == '.') {
            return format!("{}.xxx", prefix);
        }
    }
    ip.to_string()
}

/// Mask a string, showing only first and last N characters
pub fn mask_string(s: &str, visible_chars: usize) -> String {
    if s.len() <= visible_chars * 2 {
        return "*".repeat(s.len());
    }
    
    let start: String = s.chars().take(visible_chars).collect();
    let end: String = s.chars().rev().take(visible_chars).collect::<Vec<_>>().into_iter().rev().collect();
    let middle_len = s.len() - visible_chars * 2;
    
    format!("{}{}{}",start, "*".repeat(middle_len.min(8)), end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redact_sensitive() {
        let input = "connecting with password=mysecretpass";
        let redacted = redact_sensitive(input);
        assert!(redacted.contains("[REDACTED]"));
        assert!(!redacted.contains("mysecretpass"));
    }

    #[test]
    fn test_redact_no_sensitive() {
        let input = "connecting to server example.com";
        let redacted = redact_sensitive(input);
        assert_eq!(redacted, input);
    }

    #[test]
    fn test_redact_ip() {
        assert_eq!(redact_ip("192.168.1.100"), "192.168.1.xxx");
        assert_eq!(redact_ip("example.com"), "example.com");
    }

    #[test]
    fn test_mask_string() {
        assert_eq!(mask_string("verylongsecretkey", 3), "ver********key");
        assert_eq!(mask_string("short", 3), "*****");
    }
}
