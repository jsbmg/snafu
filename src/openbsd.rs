use std::process::Command;

use chrono::{Duration, Local};

/// Returns a string representing the current battery status.
pub fn battery_status(value: &str) -> Result<String, Box<dyn std::error::Error>> {
    let code = match &*value.trim() {
        "0" => { "Disconnected" },
        "1" => { "Connected" },
        "2" => { "Backup" },
        _   => { "Unknown" },
    };
    Ok(code.to_string())
}

/// Returns a string representing the charge percent of the battery
/// represented as an integer.
pub fn battery_percent(value: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let result = value.trim().parse::<i32>()?;
    Ok(result)
}

/// Retrieves battery status information in raw form.
///
/// Returns a vector with three elements at the following indices:
/// 0: battery percent
/// 1: minutes remaining
/// 2: status (connected, disconnected, etc)
pub fn _battery_one() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let command = Command::new("apm")
        .arg("-a")
        .arg("-m")
        .arg("-l")
        .output()?;

    let result: Vec<String> = String::from_utf8(command.stdout)?
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    Ok(result)
}

/// Returns the estimated battery time remaining in HH:MM format.
pub fn time_remaining(minutes: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = minutes.trim().parse::<i64>();
    let output = match output {
        Ok(x) => x,
        Err(e) => return Err(Box::new(e)),
    };
    let duration = Duration::minutes(output);
    let today = (Local::now() - Duration::days(1)).date().and_hms(0, 0, 0);
    let time = (today + duration).format("%R").to_string();
    Ok(time)
}

/// Battery percent, status, and remaining time combined.
pub fn battery_all() -> Option<String> {
    let mut result = String::new();
    let battery = match _battery_one() {
        Ok(x) => x,
        Err(_) => return None,
    };

    match battery_percent(&battery[0]) {
        Ok(percent) => result += &format!("{}% ", percent),
        Err(e) => result += &format!("Capacity: {} ", e),
    }

    match battery_status(&battery[2]) {
        Ok(status) => {
            match time_remaining(&battery[1]) {
                Ok(x) => result += &format!("[{}: {}]", status, x),
                Err(_) => result += &format!("[{}]", status),
            }
        },
        Err(e) => result += &format!("[Status: {}]", e),
    }

    match result.len() {
        0 => None,
        _ => Some(result),
    }
}

/// The ssid of the given interface, if that interface is up.
pub fn wifi(device: &str) -> Result<String, Box<dyn std::error::Error>> {
    let command = Command::new("ifconfig").arg(device).output()?;
    let stdout = String::from_utf8(command.stdout)?;

    let mut status = "";
    let mut ssid = "";

    for line in stdout.lines() {
        let line = line.trim();
        if line.starts_with("status") {
            status = line.trim_start_matches("status: ");
        } else if line.starts_with("ieee80211: join ") {
            let line = line
                .trim_start_matches("ieee80211: join ")
                .split_once(" ");
            match line {
                Some((x, _)) => { ssid = x },
                None => { ssid = "" },
            }
        }
    }
    if status == "active" && !ssid.is_empty() {
        Ok(format!("{}", ssid))
    } else {
        Ok("WiFi: Down".to_string())
    }
}
