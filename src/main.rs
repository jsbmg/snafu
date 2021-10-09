use std::error;
use std::fs::File;
use std::io::Read;
use std::process::{Command, Stdio};

use chrono::{Local};

// configuration // 
const BATTERY_NAME_OVERRIDE: Option<&str> = None;     // the optional battery device name        
const WIFI_DEVICE_OVERRIDE: Option<&str> = None;      // the optional wifi device name

const SEPARATOR: &str = " · ";                        // the separator between entries
const TIME_FORMAT: &str = "%b %d %l:%M %p";           // format for the time


// available modules: battery_capacity, battery_status, battery_all, wifi, time
const MODULES: [&str; 3] = ["battery_all", "wifi", "time"];     // the order of modules

// utilities //
fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;
    Ok(contents)
}

fn find_path_with_prefix(dir: &str, prefix: &str) -> Result<Option<String>, std::io::Error> {
    for entry in std::fs::read_dir(dir)? {
	let path = entry?.path();
	let path_name = match path.file_name() {
	    Some(f) => f,
	    None => return Ok(None),
	};
	match path_name.to_str() {
	    Some(f) if f.starts_with(prefix) => return Ok(Some(f.to_string())),
	    Some(_) => (),
	    None => (),
	}
    }
    Ok(None) 
}

fn read_battery_file(file: &str) -> Result<Option<String>, std::io::Error> {
    let battery_name;
    let power_supply_path = "/sys/class/power_supply";
    
    if let Some(name) = BATTERY_NAME_OVERRIDE {
	battery_name = name.to_string();
    } else {
	battery_name = match find_path_with_prefix(power_supply_path, "BAT") {
	    Ok(Some(x)) => x,
	    Ok(None) => return Ok(None),
	    Err(e) => return Err(e),
	};
    }
    let file_path = format!("{}/{}/{}", power_supply_path, battery_name, file);
    let info = read_file(&file_path)?;
    let info = info.trim_end().to_string();
    Ok(Some(info))
}

// components //
fn battery_capacity() -> Result<Option<String>, std::io::Error> {
    match read_battery_file("capacity") {
	Ok(Some(capacity)) => Ok(Some(capacity)),
	Ok(None) => Ok(None),
	Err(e) => Err(e),
    }
}

fn battery_status() -> Result<Option<String>, std::io::Error> {
    match read_battery_file("status") {
	Ok(Some(status)) => Ok(Some(status)),
	Ok(None) => Ok(None),
	Err(e) => Err(e),
    }
}

fn battery_capacity_and_status() -> Option<String> {
    let mut result = String::new();
    
    match battery_capacity() {
	Ok(Some(capacity)) => result += &format!("{}% ", capacity),
	Ok(None) => (),
	Err(e) => result += &format!("Capacity: {} ", e),
    }

    match battery_status() {
	Ok(Some(status)) => result += &format!("[{}]", status),
	Ok(None) => (),
	Err(e) => result += &format!("[Status: {}]", e),
    }
    
    match result.len() {
	0 => None,
	_ => Some(result),
    }
}
    
fn ssid() -> Result<Option<String>, Box<dyn error::Error>> {
    let device_name;
    let ssid;
    
    if let Some(name) = WIFI_DEVICE_OVERRIDE {
	device_name = name.to_string();
    } else {
	device_name = match find_path_with_prefix("/sys/class/net", "wlp") {
	    Ok(Some(x)) => x,
	    Ok(None) => return Ok(None),
	    Err(e) => return Err(Box::new(e)),
	};
    }
    
    let output = Command::new("iw")
	.args(["dev", &device_name, "info"])
	.stdout(Stdio::piped())
	.output()?;
    let output = String::from_utf8(output.stdout)?;
    
    for line in output.lines() {
	if line.contains("ssid") {
	    ssid = line.trim_start_matches("\tssid ").to_string();
	    return Ok(Some(ssid))
	}
    }
    Ok(None)
}

fn time() -> String {
    let now = Local::now();    
    now.format(TIME_FORMAT).to_string()
}

fn add_modules() -> Vec<String> {
    let mut modules: Vec<String> = vec![];
    
    for module in MODULES {
	match module {
	    
	    "battery_all" => {
		match battery_capacity_and_status() {
		    Some(battery) => modules.push(battery),
		    None => (),
		}
	    },

	    "battery_capacity" => {
		match battery_capacity() {
		    Ok(Some(battery)) => modules.push(battery),
		    Ok(None) => (),
		    Err(e) => modules.push(e.to_string()),
		}
	    }

	    "battery_status" => {
		match battery_status() {
		    Ok(Some(battery)) => modules.push(battery),
		    Ok(None) => (),
		    Err(e) => modules.push(e.to_string()),
		}
	    }
	    
	    "wifi" =>
		match ssid() {
		Ok(Some(ssid)) => modules.push(ssid),
		Ok(None) => (),
		Err(e) => modules.push(format!("WiFi: {}", e.to_string())),
	    },
	
	    "time" => modules.push(time()),

	    &_ => (), 
	}
    }
    modules
}

fn main() {
    let mut status_bar = String::new();
    let modules = add_modules();
    let num_modules = modules.len();
    
    if num_modules == 0 {
	println!("Status: Empty");
    } else {
	status_bar += " ";
	for idx in 0..num_modules {
	    status_bar += &modules[idx];
	    if idx < num_modules - 1 {
		status_bar += SEPARATOR;
	    }
	}
	status_bar += " ";
	
	println!("{}", status_bar);
    }
}
