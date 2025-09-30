use chrono::{Local, TimeZone, Utc};
use crate::parser::parse_iso_timestamp;

/// Convert ISO timestamp to epoch and local time
pub fn iso_to_epoch_and_local(iso: &str) -> Result<(i64, String), String> {
    let dt = parse_iso_timestamp(iso)?;
    let epoch = dt.timestamp();
    let local = dt.with_timezone(&Local);
    let local_str = local.format("%Y-%m-%d %H:%M:%S %Z").to_string();
    
    Ok((epoch, local_str))
}

/// Convert epoch timestamp to ISO and local time
pub fn epoch_to_iso_and_local(epoch: i64) -> Result<(String, String), String> {
    let dt = Utc.timestamp_opt(epoch, 0)
        .single()
        .ok_or("Invalid epoch timestamp")?;
    
    let iso = dt.to_rfc3339();
    let local = dt.with_timezone(&Local);
    let local_str = local.format("%Y-%m-%d %H:%M:%S %Z").to_string();
    
    Ok((iso, local_str))
}

/// Get current time in all formats
pub fn current_time() -> (String, i64, String) {
    let now = Utc::now();
    let epoch = now.timestamp();
    let local = now.with_timezone(&Local);
    
    let iso = now.to_rfc3339();
    let local_str = local.format("%Y-%m-%d %H:%M:%S %Z").to_string();
    
    (iso, epoch, local_str)
}

/// Auto-detect timestamp type and convert accordingly
pub fn auto_convert(input: &str) -> Result<(), String> {
    if let Ok(epoch) = input.parse::<i64>() {
        // Input looks like an epoch timestamp
        match epoch_to_iso_and_local(epoch) {
            Ok((iso, local)) => {
                println!("Epoch: {}", epoch);
                println!("ISO: {}", iso);
                println!("Local: {}", local);
                Ok(())
            }
            Err(e) => Err(e)
        }
    } else {
        // Input looks like an ISO timestamp
        match iso_to_epoch_and_local(input) {
            Ok((epoch, local)) => {
                println!("ISO: {}", input);
                println!("Epoch: {}", epoch);
                println!("Local: {}", local);
                Ok(())
            }
            Err(e) => Err(e)
        }
    }
}
