use chrono::{DateTime, Utc};

/// Parse ISO timestamp with flexible format support
pub fn parse_iso_timestamp(iso: &str) -> Result<DateTime<Utc>, String> {
    // Try parsing as RFC3339 first
    if let Ok(dt) = DateTime::parse_from_rfc3339(iso) {
        return Ok(dt.with_timezone(&Utc));
    }
    
    // Try parsing as RFC2822
    if let Ok(dt) = DateTime::parse_from_rfc2822(iso) {
        return Ok(dt.with_timezone(&Utc));
    }
    
    // Try parsing common formats
    let formats = [
        "%Y-%m-%dT%H:%M:%S%.fZ",
        "%Y-%m-%dT%H:%M:%SZ",
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M:%S UTC",
        "%Y-%m-%d %H:%M:%S GMT",
    ];
    
    for format in &formats {
        if let Ok(dt) = DateTime::parse_from_str(iso, format) {
            return Ok(dt.with_timezone(&Utc));
        }
        // Also try parsing as naive datetime and assume UTC
        if let Ok(naive_dt) = chrono::NaiveDateTime::parse_from_str(iso, format) {
            return Ok(naive_dt.and_utc());
        }
    }
    
    Err(format!("Unable to parse ISO timestamp: {}. Supported formats include RFC3339 (2023-12-25T10:30:00Z), RFC2822, and common variations.", iso))
}
