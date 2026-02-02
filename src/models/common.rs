use chrono::{DateTime, Utc};

/// Format a timestamp for human-readable display
pub fn format_timestamp(timestamp: &Option<DateTime<Utc>>) -> String {
    match timestamp {
        Some(dt) => dt.format("%Y-%m-%d %H:%M").to_string(),
        None => "-".to_string(),
    }
}

/// Format a timestamp string for display
pub fn format_timestamp_str(timestamp: &str) -> String {
    DateTime::parse_from_rfc3339(timestamp)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|_| timestamp.to_string())
}

/// Truncate a string to max length with ellipsis
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
